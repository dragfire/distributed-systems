use clap::{App, Arg, SubCommand};
use slog::*;
use std::env;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::process::exit;
use yakv::{Command, KvStore, Payload, PayloadType, Result, YakvEngine, YakvMessage};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("set")
                .arg(
                    Arg::with_name("set")
                        .value_names(&["KEY", "VALUE"])
                        .takes_value(true)
                        .number_of_values(2),
                )
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .value_name("IP-PORT")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("KEY").takes_value(true).required(true))
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .value_name("IP-PORT")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(Arg::with_name("KEY").takes_value(true).required(true))
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .value_name("IP-PORT")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    let addr: &str;
    let cmd: Command;
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let vals: Vec<_> = _matches
                .values_of("set")
                .unwrap()
                .map(ToOwned::to_owned)
                .collect();
            addr = _matches.value_of("addr").expect("Address arg is required");
            cmd = Command::set(vals[0].to_string(), vals[1].to_string());
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("KEY").map(ToOwned::to_owned).unwrap();
            addr = _matches.value_of("addr").expect("Address arg is required");
            cmd = Command::get(key);
        }
        ("rm", Some(_matches)) => {
            let key = _matches.value_of("KEY").map(ToOwned::to_owned).unwrap();
            addr = _matches.value_of("addr").expect("Address arg is required");
            cmd = Command::remove(key);
        }
        _ => unreachable!(),
    }

    // construct command and send it to server
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(&YakvMessage::get_len_payload_bytes(Payload::Command(cmd))?.1)?;
    stream.flush()?;
    let response = YakvMessage::new(&mut stream, PayloadType::Response)?;

    if let Payload::Response(res) = response.payload {
        let val = if res.is_error {
            res.error_msg
        } else {
            res.result
        };

        if res.is_error {
            eprintln!("{}", val.expect("No error message provided"));
            exit(1);
        } else if val.is_some() {
            println!("{}", val.expect("Value did not return"));
        }
    }
    Ok(())
}

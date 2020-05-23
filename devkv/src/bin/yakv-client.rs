use clap::{App, Arg, SubCommand};
use std::env;
use std::process::exit;
use yakv::{KvStore, Result, YakvEngine};

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

    let mut store = KvStore::open(env::current_dir().unwrap()).unwrap();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let vals: Vec<_> = _matches
                .values_of("set")
                .unwrap()
                .map(ToOwned::to_owned)
                .collect();
            store.set(vals[0].to_string(), vals[1].to_string())?;
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("KEY").map(ToOwned::to_owned).unwrap();
            match store.get(key) {
                Ok(val) => {
                    match val {
                        Some(v) => println!("{}", v),
                        None => println!("Key not found"),
                    }
                    exit(0)
                }
                Err(e) => {
                    println!("{:?}", e);
                    exit(1)
                }
            }
        }
        ("rm", Some(_matches)) => {
            let key = _matches.value_of("KEY").map(ToOwned::to_owned).unwrap();
            match store.remove(key) {
                Ok(_) => exit(0),
                Err(e) => {
                    println!("{:?}", e);
                    exit(1)
                }
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

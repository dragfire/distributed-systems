use clap::{App, Arg, SubCommand};
use std::process::exit;
use yakv::KvStore;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("set").arg(
                Arg::with_name("set")
                    .value_names(&["KEY", "VALUE"])
                    .takes_value(true)
                    .number_of_values(2),
            ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("KEY").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm").arg(Arg::with_name("KEY").takes_value(true).required(true)),
        )
        .get_matches();

    let mut store = KvStore::new().unwrap();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let vals: Vec<_> = _matches.values_of("set").unwrap().map(ToOwned::to_owned).collect();
            match store.set(vals[0].to_string(), vals[1].to_string()) {
                Ok(_) => {
                    exit(0)
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    exit(1)
                }
            }
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
                Ok(_) => {
                    exit(0)
                }
                Err(e) => {
                    println!("{:?}", e);
                    exit(1)
                }
            }
        }
        _ => unreachable!(),
    }
}

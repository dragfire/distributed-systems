use clap::{App, Arg};
use std::env;
use yakv::{KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .value_name("IP-PORT")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .value_name("ENGINE-NAME")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    match matches.subcommand() {
        ("addr", Some(_matches)) => {}
        ("engine", Some(_matches)) => {}
        _ => unreachable!(),
    }

    Ok(())
}

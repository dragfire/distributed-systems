use clap::{App, Arg, SubCommand};
use std::process::exit;

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

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}

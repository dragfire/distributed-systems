use clap::{App, Arg};
use slog::*;
use std::env;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use yakv::{Result, YakvEngine, YakvError};

#[derive(Debug)]
enum Engine {
    Yakv,
    Sled,
}

// NOTE: look into arg_enum!() macro from clap as an alternative
impl FromStr for Engine {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s {
            "yakv" => Ok(Engine::Yakv),
            "sled" => Ok(Engine::Sled),
            _ => Err(()),
        }
    }
}

// NOTE: look into structopt
#[derive(Debug)]
struct Config {
    addr: SocketAddr,
    engine: Engine,
}

fn main() -> Result<()> {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
    info!(log, "world");

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

    let addr = matches.value_of("addr").expect("ADDR arg is required");
    let engine = matches.value_of("engine").expect("ENGINE arg is required");
    let config = Config {
        addr: SocketAddr::from_str(addr).expect("Address is not a valid IPV4 address."),
        engine: Engine::from_str(engine).expect("Use either sled or yakv as ENGINE value"),
    };

    // start server
    start_server(config)?;

    Ok(())
}

fn start_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(config.addr)?;
    for stream in listener.incoming() {
        handle_request(stream?)?;
    }
    Ok(())
}

fn handle_request(stream: TcpStream) -> Result<()> {
    let mut stream_mut = stream;
    stream_mut.write(b"Ok")?;
    Ok(())
}

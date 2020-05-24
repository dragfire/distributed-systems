use clap::{App, Arg};
use slog::*;
use std::env;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use yakv::{Command, Payload, PayloadType, Result, YakvEngine, YakvError, YakvMessage};

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

struct YakvServer {
    config: Config,
    log: slog::Logger,
}

impl YakvServer {
    fn new(config: Config, log: slog::Logger) -> Self {
        YakvServer { config, log }
    }

    fn start(&mut self) -> Result<()> {
        info!(self.log, "config: {:?}", self.config);
        let listener = TcpListener::bind(&self.config.addr)?;
        for stream in listener.incoming() {
            let tcp_stream = stream?;
            info!(self.log, "connection accepted");
            self.handle_request(tcp_stream)?;
        }
        Ok(())
    }

    fn handle_request(&mut self, stream: TcpStream) -> Result<()> {
        let mut stream = stream;
        let message = YakvMessage::new(&mut stream, PayloadType::Command)?;
        info!(self.log, "Req: {:?}", message);
        stream.write_all(&[1])?;
        stream.flush()?;
        Ok(())
    }

    fn get_message(&mut self, reader: &mut TcpStream) -> Result<Command> {
        let mut req_str = String::new();
        reader.read_to_string(&mut req_str)?;
        Ok(serde_json::from_str::<Command>(&req_str)?)
    }
}

fn main() -> Result<()> {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();

    let log = slog::Logger::root(drain, o!());
    info!(log, "version: {}", env!("CARGO_PKG_VERSION"));

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
    let mut server = YakvServer::new(config, log);
    server.start()?;

    Ok(())
}

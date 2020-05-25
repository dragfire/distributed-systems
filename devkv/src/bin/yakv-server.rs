use anyhow::*;
use clap::{App, Arg};
use slog::*;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};
use std::iter::Iterator;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use yakv::{
    Command, KvStore, Payload, PayloadType, Response, Result, YakvEngine, YakvError, YakvMessage,
};

#[derive(Debug, PartialEq, Eq, Hash)]
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

struct YakvServer<E> {
    config: Config,
    log: slog::Logger,
    store: E,
}

impl<E: YakvEngine> YakvServer<E> {
    fn new(config: Config, log: slog::Logger, store: E) -> Self {
        YakvServer { config, log, store }
    }

    fn start(&mut self) -> Result<()> {
        info!(self.log, "engine: {:?}", self.config.engine);
        info!(self.log, "ip: {:?}", self.config.addr);
        let listener = TcpListener::bind(&self.config.addr)?;
        for stream in listener.incoming() {
            let mut tcp_stream = stream?;
            info!(self.log, "connection accepted");
            match self.handle_request(&mut tcp_stream) {
                Ok(res) => {
                    self.send_response(&mut tcp_stream, res)?;
                }
                Err(e) => {
                    error!(self.log, "{:?}", e);
                    let res = Response::new(true, Some(e.to_string()), None);
                    self.send_response(&mut tcp_stream, res)?;
                }
            }
        }
        Ok(())
    }

    fn send_response(&mut self, stream: &mut TcpStream, res: Response) -> Result<()> {
        let (_, bytes) = YakvMessage::get_len_payload_bytes(Payload::Response(res))?;
        stream.write_all(&bytes)?;
        stream.flush()?;
        Ok(())
    }

    fn handle_request(&mut self, mut stream: &mut TcpStream) -> Result<Response> {
        let message = YakvMessage::new(&mut stream, PayloadType::Command)?;
        info!(self.log, "Req: {:?}", message.payload);
        let mut response: Response = Default::default();

        if let Payload::Command(cmd) = message.payload {
            match cmd {
                Command::Set { key, value } => {
                    self.store.set(key, value)?;
                }
                Command::Get { key } => {
                    response = Response::new(false, None, self.store.get(key)?);
                }
                Command::Remove { key } => {
                    self.store.remove(key)?;
                }
            }
        }

        Ok(response)
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
                .default_value("yakv"),
        )
        .get_matches();

    let addr = matches.value_of("addr").expect("ADDR arg is required");
    let engine_arg = matches.value_of("engine").expect("ENGINE arg is required");
    let config = Config {
        addr: SocketAddr::from_str(addr).expect("Address is not a valid IPV4 address."),
        engine: Engine::from_str(engine_arg).unwrap_or(Engine::Yakv),
    };

    let current_dir = env::current_dir()?;
    let existing_engines = get_existing_engines(current_dir.clone())?;
    if !existing_engines.is_empty() && !existing_engines.contains(&config.engine) {
        return Err(YakvError::Any(anyhow!(
            "Engine value is different from already used engines."
        )));
    }
    let store = match config.engine {
        Engine::Yakv => KvStore::open(current_dir)?,
        Engine::Sled => KvStore::open(current_dir)?,
    };

    // start server
    let mut server = YakvServer::new(config, log, store);
    server.start()?;

    Ok(())
}

fn get_existing_engines(path: PathBuf) -> Result<HashSet<Engine>> {
    let existing_engines = fs::read_dir(path)?
        .flat_map(|dir| dir.map(|e| e.path()))
        .filter_map(|e| {
            e.file_stem()
                .and_then(OsStr::to_str)
                .filter(|s| s.starts_with("engine"))
                .map(|s| &s[7..11])
                .map(Engine::from_str)
        })
        .flatten()
        .collect();
    Ok(existing_engines)
}

#[test]
fn test_get_existing_engines() {
    use tempfile::Builder;
    let tmp_dir = Builder::new().tempdir().unwrap();
    let path = tmp_dir.into_path();
    let mut data_path = path.clone();
    data_path.push("engine_yakv_data");
    fs::create_dir_all(data_path).unwrap();
    let engines = get_existing_engines(path).unwrap();
    assert!(engines.iter().eq(vec![Engine::Yakv].iter()))
}

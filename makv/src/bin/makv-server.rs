use anyhow::*;
use clap::{App, Arg};
use makv::{
    Command, Engine, KvStore, MakvEngine, Payload, PayloadType, Response, Result, YakvError,
    YakvMessage,
};
use slog::*;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::iter::Iterator;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::str::FromStr;

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

impl<E: MakvEngine> YakvServer<E> {
    fn new(config: Config, log: slog::Logger, store: E) -> Self {
        YakvServer { config, log, store }
    }

    fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.config.addr)?;
        for stream in listener.incoming() {
            let store = self.store.clone();
            if let Ok(tcp_stream) = stream {
                match handle_request(&tcp_stream, store) {
                    Ok(res) => {
                        send_response(&tcp_stream, res)?;
                    }
                    Err(e) => {
                        let res = Response::new(true, Some(e.to_string()), None);
                        send_response(&tcp_stream, res)?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn send_response(mut stream: &TcpStream, res: Response) -> Result<()> {
    let (_, bytes) = YakvMessage::get_len_payload_bytes(Payload::Response(res))?;
    stream.write_all(&bytes)?;
    stream.flush()?;
    Ok(())
}

fn handle_request<E: MakvEngine>(mut stream: &TcpStream, store: E) -> Result<Response> {
    let message = YakvMessage::new(&mut stream, PayloadType::Command)?;
    let mut response: Response = Default::default();

    if let Payload::Command(cmd) = message.payload {
        match cmd {
            Command::Set { key, value } => {
                store.set(key, value)?;
            }
            Command::Get { key } => {
                response = Response::new(
                    false,
                    None,
                    store.get(key)?.or(Some("Key not found".to_string())),
                );
            }
            Command::Remove { key } => {
                store.remove(key)?;
            }
        }
    }

    Ok(response)
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

    match config.engine {
        Engine::Yakv => {
            let store = KvStore::open(current_dir)?;
            let server = YakvServer::new(config, log, store);
            server.start()?;
        }
        Engine::Sled => {}
    };

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

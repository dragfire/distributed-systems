use serde::{Deserialize, Serialize};
use serde_json::{self, Deserializer};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Debug, Deserialize)]
enum Command {
    Set(String, String),
    Get(String),
    Remove(String),
}

fn main() {
    let path = PathBuf::from("./test.log");
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&path)
        .unwrap();
    let mut writer = BufWriter::new(&file);
    let cmd1 = Command::Set("key".to_string(), "value".to_string());
    writer
        .write_all(&serde_json::to_vec(&cmd1).unwrap())
        .unwrap();
    writer.flush().unwrap();
    drop(writer);
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&path)
        .unwrap();

    let reader = BufReader::new(&file);
    let cmd_reader = Deserializer::from_reader(reader).into_iter::<Command>();

    for cmd in cmd_reader {
        println!("{:?}", cmd);
    }
}

use crate::{Command, Result, YakvError};
use anyhow::*;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(missing_docs)]
pub enum PayloadType {
    Command,
    Response,
    Empty,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Payload {
    Command(Command),
    Response(String),
    Empty,
}

/// Represents YakvMessage that will be used to communicate in TCPStream
///
#[derive(Debug)]
pub struct YakvMessage {
    /// length of payload
    pub length: u32,

    /// payload for the message
    pub payload: Payload,
}

impl YakvMessage {
    /// Prepend length: 4 bytes to payload
    pub fn get_bytes(cmd: Command) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = serde_json::to_vec::<Command>(&cmd)?;
        let len = bytes.len();
        let mut len_bytes: Vec<u8> = len.to_be_bytes().to_vec();
        bytes.append(&mut len_bytes);
        Ok(bytes)
    }

    /// Bytes to YakvMessage
    fn get_payload_bytes(stream: &mut TcpStream) -> Result<(u32, Vec<u8>)> {
        let mut len_buf: [u8; 4] = [0; 4];
        let mut handle = stream.take(4);
        if handle.limit() != 4 {
            return Err(YakvError::Any(anyhow!("Payload needs to be 4 bytes long.")));
        }
        handle.read_exact(&mut len_buf)?;
        let length = u32::from_be_bytes(len_buf);
        let mut payload_buf = vec![0; length as usize];
        stream.read_exact(&mut payload_buf)?;
        Ok((length, payload_buf))
    }

    /// Returns payload from TcpStream and handle different payload types.
    pub fn new(stream: &mut TcpStream, ptype: PayloadType) -> Result<Self> {
        let (length, buf) = YakvMessage::get_payload_bytes(stream)?;
        let payload: Payload;
        match ptype {
            PayloadType::Command => {
                payload = Payload::Command(serde_json::from_slice::<Command>(&buf)?);
            }
            PayloadType::Response => {
                let value = String::from_utf8(buf).expect("Value needs to be valid bytes");
                payload = Payload::Response(value);
            }
            PayloadType::Empty => {
                payload = Payload::Empty;
            }
        }
        Ok(YakvMessage { length, payload })
    }
}

#[test]
fn test_message_from_bytes() {
    let msg = YakvMessage {
        length: 4,
        payload: Payload::Empty,
    };
}

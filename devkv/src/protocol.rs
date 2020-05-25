use crate::{Command, Result, YakvError};
use anyhow::*;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Represents different Payload types.
pub enum PayloadType {
    /// Command variant
    Command,

    /// When sending reponse back to client
    Response,

    /// Empty reponse is possible
    /// in case of Command::Set or Command::Remove
    Empty,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Payload {
    Command(Command),
    Response(String),
    Empty,
}

/// Represents networking protocol
///
/// Since TCP needs a way to distinguish how many bytes are actually needed
/// to read and write, a custom protocol helps us solve this problem.
///
/// Each read or write to TcpStream will utilize YakvMessage.
/// This struct has the length of the actual payload we are sending over the
/// network. This lets the protocol know how much bytes it needs for the buffer.
///
/// We can find out the length of the payload from first 4 bytes i.e. [u8; 4]
#[derive(Debug)]
pub struct YakvMessage {
    /// length of payload
    pub length: u32,

    /// payload for the message
    pub payload: Payload,
}

impl YakvMessage {
    /// Get length of the payload and convert payload to bytes
    ///
    /// Prepend length: 4 bytes to payload bytes
    /// Returns (length, payload_bytes)
    pub fn get_len_payload_bytes(payload: Payload) -> Result<(u32, Vec<u8>)> {
        let mut bytes: Vec<u8> = vec![];
        match payload {
            Payload::Command(cmd) => {
                bytes = serde_json::to_vec::<Command>(&cmd)?;
            }
            Payload::Response(val) => {
                // do
                bytes = val.into_bytes();
            }
            _ => {}
        }
        let len = bytes.len() as u32;
        let mut len_bytes = len.to_be_bytes().to_vec();
        len_bytes.append(&mut bytes);
        Ok((len, len_bytes))
    }

    fn get_stream_payload_bytes(stream: &mut TcpStream) -> Result<(u32, Vec<u8>)> {
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
        let (length, buf) = YakvMessage::get_stream_payload_bytes(stream)?;
        let payload: Payload;
        match ptype {
            PayloadType::Command => {
                payload = Payload::Command(serde_json::from_slice::<Command>(&buf)?);
            }
            PayloadType::Response => {
                if length == 0 {
                    payload = Payload::Empty;
                } else {
                    let value = String::from_utf8(buf).expect("Value needs to be valid bytes");
                    payload = Payload::Response(value);
                }
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
    let cmd = Command::remove("key".to_string());
    let (length, bytes) = YakvMessage::get_len_payload_bytes(Payload::Command(cmd)).unwrap();
    let len_bytes: [u8; 4] = bytes[..4].try_into().unwrap();
    let actual_len = u32::from_be_bytes(len_bytes);
    assert_eq!(length, actual_len);

    let res_payload = Payload::Response("OK".to_string());
    let (length, bytes) = YakvMessage::get_len_payload_bytes(res_payload).unwrap();
    let len_bytes = bytes[..4].try_into().expect("Valid 4 bytes required");
    let actual_len = u32::from_be_bytes(len_bytes);
    assert_eq!(length, actual_len);

    let str_bytes: Vec<u8> = bytes[4..].try_into().unwrap();
    assert_eq!("OK".to_string(), String::from_utf8(str_bytes).unwrap());
}

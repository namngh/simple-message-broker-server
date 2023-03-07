use std::{error::Error, str};

use serde_derive::{Deserialize, Serialize};

pub enum ProtocolMessage {
    INFO,
    CONNECT,
    PUB,
    SUB,
    MSG,
    PING,
    PONG,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionPayload {
    auth_token: String,
    user: String,
    pass: String,
}

pub struct ProtocolConnect {
    payload: ConnectionPayload,
}

pub fn parse(first_byte: u8) -> Option<ProtocolMessage> {
    match first_byte {
        1 => Some(ProtocolMessage::CONNECT),
        2 => Some(ProtocolMessage::PUB),
        3 => Some(ProtocolMessage::SUB),
        5 => Some(ProtocolMessage::PING),
        6 => Some(ProtocolMessage::PONG),
        _ => None,
    }
}

pub fn parse_connect(data: Vec<u8>) -> Result<ProtocolConnect, Box<dyn Error>> {
    let json_str = str::from_utf8(&data[1..])?;
    let payload: ConnectionPayload = serde_json::from_str(json_str)?;

    Ok(ProtocolConnect { payload: payload })
}

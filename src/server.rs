use std::{
    error::Error,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::errors;
use crate::protocol;

pub fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        let bytes_read = stream.read_until(b'\n', &mut data)?;

        if bytes_read == 0 {
            return Ok(());
        }

        match protocol::parse(data[0]) {
            Some(protocol::ProtocolMessage::CONNECT) => {
                protocol::parse_connect(data);
                return Ok(());
            }
            Some(protocol::ProtocolMessage::PUB) => {
                protocol::parse_connect(data);
                return Ok(());
            }
            Some(protocol::ProtocolMessage::SUB) => {
                protocol::parse_connect(data);
                return Ok(());
            }
            Some(protocol::ProtocolMessage::PING) => {
                protocol::parse_connect(data);
                return Ok(());
            }
            Some(protocol::ProtocolMessage::PONG) => {
                protocol::parse_connect(data);
                return Ok(());
            }
            _ => {
                return Err(Box::new(errors::SimpleMessageBrokerServerError(
                    "Invalid protocol message".into(),
                )))
            }
        }

        // stream.write(&buf[..bytes_read])?;
    }
}

use std::{
    collections::HashMap,
    error::Error,
    net::{SocketAddr, TcpStream},
};

use fastuuid::Generator;

pub struct Connection {
    subject: HashMap<String, &Vec<String>>,
    peer: HashMap<String, TcpStream>,
}

pub fn new() -> Connection {
    Connection {
        subject: HashMap::new(),
        peer: HashMap::new(),
    }
}

pub fn add_peer(
    conn: &mut Connection,
    socket_addr: SocketAddr,
    subject: String,
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(socket_addr)?;

    let generator = Generator::new();
    let uuid = generator.hex128_as_string().unwrap();

    conn.peer.insert(uuid, stream);

    let subject_peer = match conn.subject.get(&subject) {
        Some(v) => {
            v.push(uuid);
            v
        }
        None => &vec![uuid],
    };
    conn.subject.insert(subject, subject_peer);

    Ok(())
}

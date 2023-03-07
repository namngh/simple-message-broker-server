mod connection;
mod constants;
mod errors;
mod protocol;
mod server;

use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
    thread,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "simple-message-broker-server - a simple CLI to start and customize its behaviour",
    long_about = "simple-message-broker-server is a small server to demonstrate how message broker work"
)]
struct Cli {
    #[arg(short, long)]
    addr: Option<String>,

    #[arg(short, long)]
    port: Option<String>,

    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let address = cli.addr.unwrap_or(constants::DEFAULt_ADDRESS.to_string());
    let port = cli.port.unwrap_or(constants::DEFAULt_PORT.to_string());
    // let configuration_path = cli
    //     .config
    //     .unwrap_or(constants::DEFAULt_CONFIGURATION_PATH.to_string());

    let socket_addr = SocketAddr::from_str(&format!("{address}:{port}")[..])
        .expect("Invalid address or port format (Ex: --addr 0.0.0.0 --port 6868)");
    let listener = TcpListener::bind(socket_addr).expect("Could not bind to TCP");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    server::handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
            }
        }
    }
}

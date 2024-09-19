use env_logger;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod commands;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    info!("Server running on port 6379");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: tokio::net::TcpStream) {
    info!("Accepted new connection");

    let mut buf = vec![0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => {
                // Connection was closed by the client
                return;
            }
            Ok(n) => n,
            Err(e) => {
                error!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let data = &buf[..n];

        match parser::parse_commands(data) {
            Ok(parsed_commands) => {
                if let Some(command) = parsed_commands.get(0) {
                    let response = commands::process_command(command.join(" "));
                    if let Err(_) = socket.write_all(response.as_bytes()).await {
                        break;
                    }
                }
            }
            Err(err) => {
                let err_response = format!("-ERR {}\r\n", err);
                error!("{}", err);
                if let Err(_) = socket.write_all(err_response.as_bytes()).await {
                    break;
                }
            }
        }
    }
}

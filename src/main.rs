use db::Db;
use env_logger;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

mod commands;
mod db;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:6379").await?;
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    info!("Server running on port 6379");

    loop {
        let (socket, _) = listener.accept().await?;
        let db_thread = Arc::clone(&db);
        tokio::spawn(async move {
            handle_client(socket, db_thread).await;
        });
    }
}

async fn handle_client(mut socket: tokio::net::TcpStream, db: Db) {
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
                for command in parsed_commands {
                    let response = commands::process_command(command.join(" "), &db).await;
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

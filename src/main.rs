use db::Db;
use env_logger;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::available_parallelism;

use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod commands;
mod db;
mod parser;

/// Starts the server and logging in a synchronous thread.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let listener = std::net::TcpListener::bind("0.0.0.0:6379")?;
    let db: Db = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

    info!("Server running on port 6379");

    let max_threads = available_parallelism().unwrap().get();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(max_threads)
        .enable_all()
        .build()?;

    runtime.block_on(async {
        start_tokio_requests_handling(listener, db).await?;
        Ok(())
    })
}

/// Handles incoming connections with parallelism up to the max number of available threads.
async fn start_tokio_requests_handling(
    listener: std::net::TcpListener,
    db: Db,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::from_std(listener)?;

    loop {
        let (socket, _) = listener.accept().await?;

        // Only clones a pointer to the db, not the db itself
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

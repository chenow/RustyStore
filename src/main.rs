use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = [0; 64];
                loop {
                    let nb_bytes = stream.read(&mut buf).unwrap();
                    if nb_bytes == 0 {
                        // TCP connection closed
                        break;
                    }
                    stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

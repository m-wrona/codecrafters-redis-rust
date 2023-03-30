// Uncomment this block to pass the first stage
use std::{io::Write, net::TcpListener};
use std::io::Read;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = [0; 512];
                _stream.read(&mut buf).unwrap();
                let response = "+PONG\r\n";
                _stream.write_all(response.as_bytes()).expect("ping error")
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(connection) => handle_connection(connection),
            Err(e) => println!("{e}"),
        }
    }
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut tcp_stream);
    let _: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nX-Custom-Header: simple server\r\n\r\nsimple text";
    let result = tcp_stream.write_all(response.as_bytes());
    println!("{:?}", result.unwrap());
}

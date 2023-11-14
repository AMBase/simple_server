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
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = handle_request(request).unwrap();

    let mut data = String::new();
    match response.status {
        200 => data.push_str("HTTP/1.1 200 OK\r\n"),
        _ => data.push_str("HTTP/1.1 500 Internal Server Error\r\n"),
    }

    data.push_str(&response.headers.join("\r\n"));

    data.push_str("\r\n\r\n");
    data.push_str(&response.body);

    let result = tcp_stream.write_all(data.as_bytes());
    println!("{:#?}", result.unwrap());
}

struct Response {
    status: u8,
    headers: Vec<String>,
    body: String,
}


fn handle_request(request: Vec<String>) -> Result<Response, String> {
    println!("{request:#?}");

    let status = 200;
    let headers = vec![
        String::from("Content-Type: text/plain"),
        String::from("X-Custom-Header: simple server"),
    ];
    let body = String::from("simple text");
    let response = Response {status, headers, body};

    Ok(response)
}

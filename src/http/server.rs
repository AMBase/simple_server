

use std::net::{ToSocketAddrs, TcpListener, TcpStream};

use std::io::Write;


use std::io::prelude::*;


use crate::http::{Request, Response};

pub struct Server<A: ToSocketAddrs> {
    addr: A,
}

impl<A: ToSocketAddrs> Server<A> {
    pub fn new(addr: A) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        for stream in listener.incoming() {
            let connection = stream.unwrap();
            let _ = handle_connection(connection);
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

fn handle_connection(mut tcp_stream: TcpStream) -> Result<(), Error> {
    let request = read_request(&mut tcp_stream)?;

    let result = handle_request(request);
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            let data = format!("HTTP/1.1 {} {}\r\n", error.code, error.message).to_string();
            let _ = tcp_stream.write_all(data.as_bytes());

            return Ok(());
        }
    };

    let mut data = String::new();
    match response.status {
        200 => data.push_str("HTTP/1.1 200 OK\r\n"),
        _ => data.push_str("HTTP/1.1 500 Internal Server Error\r\n"),
    }

    data.push_str(&response.headers.join("\r\n"));

    data.push_str("\r\n\r\n");
    data.push_str(&response.body);

    let _ = tcp_stream.write_all(data.as_bytes());

    Ok(())
}






fn handle_request(request: Request) -> Result<Response, Error> {
    // if request.len() <= 0 {
    //     return Err(Error {code: 500, message: String::from("Internal Server Error")});
    // }

    let status = 200;
    let headers = vec![
        String::from("Content-Type: application/json"),
        String::from("X-Custom-Header: simple server"),
    ];
    let body = String::from(r#"{"data": "test"}"#);
    let response = Response {status, headers, body};

    Ok(response)
}




fn read_request(tcp_stream: &mut TcpStream) -> Result<Request, Error> {
    let mut buffer = [0u8; 2];
    let mut request = Request::new();
    request.method = super::parser::parse_method(tcp_stream, &mut buffer).unwrap();
    request.uri = super::parser::parse_uri(tcp_stream, &mut buffer).unwrap();
    request.version = super::parser::parse_version(tcp_stream, &mut buffer);

    println!("{:?}", request);
    println!("{:?}", request.method);
    // println!("{:?}", String::from_utf8_lossy(request.uri));
    // println!("{:?}", String::from_utf8_lossy(request.version));

    println!("{buffer:?}");

    Ok(request)
}

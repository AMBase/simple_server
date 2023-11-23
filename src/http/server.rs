use crate::http::parser::ParseState;

use std::net::{ToSocketAddrs, TcpListener, TcpStream};

use std::io::Write;

use std::io::BufReader;
use std::io::prelude::*;

use crate::http::request::RawRequest;
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
struct Error {
    code: u16,
    message: String,
}

fn handle_connection(mut tcp_stream: TcpStream) -> Result<(), Error> {
    let raw_request = read_request(&mut tcp_stream)?;
    let request = parse_request(&raw_request)?;

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




fn read_request(tcp_stream: &mut TcpStream) -> Result<RawRequest, Error> {
    let mut buffer = [0u8; 2];
    let mut raw_request: Vec<u8> = Vec::new();

    loop {
        let bytes_read = tcp_stream.read(&mut buffer).unwrap();

        let mut state: ParseState = ParseState::Method;
        let mut request = Request::new();

        let mut last = 0usize;
        for i in 0..=buffer.len() -1  {
            let char = buffer[i];
            println!("{char:?}");
            match state {
                ParseState::Method => {
                    if char == SP {
                        request.method = &raw_request[last..i];
                        state = ParseState::RequestURI;
                        last = i + 1;
                    }
                },
                ParseState::RequestURI => {
                    if char == SP {
                        request.uri = &raw_request[last..i];
                        state = ParseState::HTTPVersion;
                        last = i + 1;
                    }
                },
                ParseState::HTTPVersion => {
                    if char == CR {
                        request.version = &raw_request[last..i];
                        state = ParseState::End;
                        last = i;
                    }
                },
                ParseState::End => break,
            }
        }
        println!("{:?}", request);
        println!("{:?}", String::from_utf8_lossy(request.method));
        println!("{:?}", String::from_utf8_lossy(request.uri));
        println!("{:?}", String::from_utf8_lossy(request.version));

        if bytes_read < 1024 {
            break;
        }
    }


    println!("{buffer:?}");


    let mut reader = BufReader::new(tcp_stream);


    Ok(raw_request)
}


const CR: u8 = 13;
const LF: u8 = 10;
const SP: u8 = 32;




fn parse_request(raw_request: &RawRequest) -> Result<Request, Error> {
    let mut state: ParseState = ParseState::Method;
    let mut request = Request::new();

    println!("{:?}", String::from_utf8_lossy(raw_request));

    let mut last = 0usize;
    for (idx, char) in raw_request.iter().enumerate() {
        match state {
            ParseState::Method => {
                if *char == SP {
                    request.method = &raw_request[last..idx];
                    state = ParseState::RequestURI;
                    last = idx + 1;
                }
            },
            ParseState::RequestURI => {
                if *char == SP {
                    request.uri = &raw_request[last..idx];
                    state = ParseState::HTTPVersion;
                    last = idx + 1;
                }
            },
            ParseState::HTTPVersion => {
                if *char == CR {
                    request.version = &raw_request[last..idx];
                    state = ParseState::End;
                    last = idx;
                }
            },
            ParseState::End => break,
        }
    }
    println!("{:?}", request);
    println!("{:?}", String::from_utf8_lossy(request.method));
    println!("{:?}", String::from_utf8_lossy(request.uri));
    println!("{:?}", String::from_utf8_lossy(request.version));

    Ok(request)
}
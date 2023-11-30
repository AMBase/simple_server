use std::net::TcpStream;
use super::request;
use std::io::Read;
use crate::http::request::Method;

const CR: u8 = 13;
const LF: u8 = 10;
const SP: u8 = 32;

pub enum ParseState {
    Method,
    RequestURI,
    HTTPVersion,
    End,
}

pub fn parse_method(tcp_stream: &mut TcpStream, buf: &mut [u8]) -> Result<Method, super::server::Error> {
    loop {
        let bytes_read = tcp_stream.read(buf).unwrap();
        for i in 0..=buf.len() - 1 {
            let char = buf[i];

            println!("i={i}, char={char}");
            if char == SP {
                return Ok(Method::GET);
            }
        }

        if bytes_read < buf.len() {
            break;
        }
    }

    let err = super::server::Error {
        code: 400,
        message: "".to_string(),
    };
    return Err(err);
}

pub fn parse_uri(tcp_stream: &mut TcpStream, buffer: &mut [u8]) -> Result<String, super::server::Error> {
    let mut url = String::new();

    loop {
        let bytes_read = tcp_stream.read(buffer).unwrap();
        for i in 0..=buffer.len() - 1 {
            let char = buffer[i];
            url.push(char as char);

            println!("i={i}, char={char}");
            if char == SP {
                return Ok(url);
            }
        }

        if bytes_read < buffer.len() {
            break;
        }
    }

    let err = super::server::Error {
        code: 400,
        message: "".to_string(),
    };
    return Err(err);
}

pub fn parse_version(tcp_stream: &mut TcpStream, buffer: &mut [u8]) -> String {
    loop {
        let bytes_read = tcp_stream.read(buffer).unwrap();


        if bytes_read < buffer.len() {
            break;
        }
    }

    return "".to_string();
}
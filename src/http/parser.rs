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

pub fn parse_method(tcp_stream: &mut TcpStream, buf: &mut [u8]) -> Result<request::Method, super::server::Error> {
    loop {
        let bytes_read = tcp_stream.read(buf).unwrap();
        for i in 0..=buf.len() - 1 {
            let char = buf[i];

            println!("i={i}, char={char}");
            if char == 32 {
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

pub fn parse_uri(tcp_stream: &mut TcpStream, buffer: &mut [u8]) -> String {
    return "".to_string();
}

pub fn parse_version(tcp_stream: &mut TcpStream, buffer: &mut [u8]) -> String {
    return "".to_string();
}
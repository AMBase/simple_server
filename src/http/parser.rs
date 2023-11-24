use std::net::TcpStream;
use super::request;
use std::io::Read;
use crate::http::request::Method;

pub enum ParseState {
    Method,
    RequestURI,
    HTTPVersion,
    End
}

pub fn parse_method(tcp_stream: &mut TcpStream, buf: &mut [u8]) -> request::Method {
    let mut i = 0;
    loop {
        let char = buf[i];
        println!("i={i}, char={char}");

        if char == 32 {
            break;
        }
        i += 1;

        if i >= buf.len() - 1 {
            tcp_stream.read(buf).unwrap();
            i = 0;
        }
    }

    return Method::GET;
}
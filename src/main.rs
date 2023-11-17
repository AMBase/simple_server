use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Error {
    code: u16,
    message: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let connection = stream.unwrap();
        let r = handle_connection(connection);
        println!("{:?}", r.unwrap());
    }

    ()
}

fn handle_connection(mut tcp_stream: TcpStream) -> Result<(), Error> {
    let request = parse_request(&tcp_stream)?;
    println!("{:?}", request);

    // let buf_reader = BufReader::new(&tcp_stream);
    // let request: Vec<String> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let result = handle_request(request);
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            let data = format!("HTTP/1.1 {} {}\r\n", error.code, error.message).to_string();
            let result = tcp_stream.write_all(data.as_bytes());
            println!("{:#?}", result.unwrap());
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

    let result = tcp_stream.write_all(data.as_bytes());
    println!("{:#?}", result.unwrap());

    Ok(())
}

struct Response {
    status: u16,
    headers: Vec<String>,
    body: String,
}




fn handle_request(request: Request) -> Result<Response, Error> {
    println!("{request:#?}");

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
#[derive(Debug)]
struct Request {

}

fn parse_request(tcp_stream: &TcpStream) -> Result<Request, Error> {
    let request = Request {};
    let mut reader = BufReader::new(tcp_stream);
    let mut raw_request: Vec<u8> = Vec::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        raw_request.extend_from_slice(&buffer[..bytes_read]);
        if bytes_read < 1024 {
            break;
        }
    }



    println!("{:?}", raw_request);

    Ok(request)
}

use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(connection) => handle_connection(connection),
            Err(e) => println!("{e}"),
        }
    }
}

fn handle_connection(tcp_stream: TcpStream) {
    println!("{:?}", tcp_stream.local_addr())
}

use simple_server::http::server::Server;



fn main() {
    let address = "127.0.0.1:8080";
    let server = Server::new(address);
    server.run();

    ()
}

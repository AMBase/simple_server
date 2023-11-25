pub type RawRequest = Vec<u8>;

#[derive(Debug)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
    TRACE,
    CONNECT,
    LINK,
    UNLINK,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: String,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            uri: String::from(""),
            version: String::from(""),
        }
    }
}



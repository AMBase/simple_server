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
pub struct Request<'a> {
    pub method: Method,
    pub uri: &'a[u8],
    pub version: &'a[u8],
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            uri: &[0],
            version: &[0],
        }
    }
}



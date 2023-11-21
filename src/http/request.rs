pub type RawRequest = Vec<u8>;
#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a[u8],
    pub uri: &'a[u8],
    pub version: &'a[u8],
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            method: &[0],
            uri: &[0],
            version: &[0],
        }
    }
}
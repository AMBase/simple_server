pub type RawRequest = Vec<u8>;
#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a[u8],
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            method: &[0]
        }
    }
}
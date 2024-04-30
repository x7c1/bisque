use crate::schemas::Metadata;
use crate::{here, Result};
use std::io::{Cursor, Read};

pub struct Reader<A> {
    inner: A,
}

impl Reader<()> {
    pub fn new(file: impl Read, metadata: Metadata, boundary: &str) -> Result<Reader<impl Read>> {
        let boundary = Cursor::new(format!("--{boundary}"));
        let metadata = Cursor::new(serde_json::to_string(&metadata).map_err(here!())?);
        let chain = boundary
            .clone()
            .chain("\r\n".as_bytes())
            .chain("Content-Type: application/json; charset=UTF-8\r\n\r\n".as_bytes())
            .chain(metadata.chain("\r\n".as_bytes()))
            .chain(boundary.clone().chain("\r\n".as_bytes()))
            .chain("Content-Type: application/octet-stream\r\n\r\n".as_bytes())
            .chain(file)
            .chain("\r\n".as_bytes())
            .chain(boundary.chain("--".as_bytes()));

        Ok(Reader { inner: chain })
    }
}

impl<A: Read> Read for Reader<A> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

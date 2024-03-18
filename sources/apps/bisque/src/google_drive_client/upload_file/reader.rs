use crate::google_drive_client::upload_file::Metadata;
use crate::Result;
use std::fs::File;
use std::io::{Cursor, Read};

pub struct Reader {
    inner: Box<dyn Read + Send>,
}

impl Reader {
    pub fn new(file: File, metadata: Metadata, boundary_value: &str) -> Result<Reader> {
        let boundary = Cursor::new(format!("--{boundary_value}\r\n"));
        let metadata = Cursor::new(serde_json::to_string(&metadata)?);
        let reader = boundary
            .clone()
            .chain("Content-Type: application/json; charset=UTF-8\r\n\r\n".as_bytes())
            .chain(metadata)
            .chain("\r\n".as_bytes())
            .chain(boundary)
            .chain("Content-Type: application/octet-stream\r\n\r\n".as_bytes())
            .chain(file)
            .chain("\r\n".as_bytes())
            .chain(Cursor::new(format!("--{boundary_value}--")));

        Ok(Reader {
            inner: Box::new(reader),
        })
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

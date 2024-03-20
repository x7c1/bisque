use crate::google_drive_client::upload_file::Metadata;
use crate::Result;
use std::fs::File;
use std::io::{Cursor, Read};

pub struct Reader;

impl Reader {
    pub fn from(file: File, metadata: Metadata, boundary: &str) -> Result<impl Read> {
        let boundary = Cursor::new(format!("--{boundary}"));
        let metadata = Cursor::new(serde_json::to_string(&metadata)?);
        let reader = boundary
            .clone()
            .chain("\r\n".as_bytes())
            .chain("Content-Type: application/json; charset=UTF-8\r\n\r\n".as_bytes())
            .chain(metadata.chain("\r\n".as_bytes()))
            .chain(boundary.clone().chain("\r\n".as_bytes()))
            .chain("Content-Type: application/octet-stream\r\n\r\n".as_bytes())
            .chain(file)
            .chain("\r\n".as_bytes())
            .chain(boundary.chain("--".as_bytes()));

        Ok(reader)
    }
}

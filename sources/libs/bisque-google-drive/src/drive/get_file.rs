use crate::drive::GoogleDriveClient;
use crate::{here, Result};
use std::io::Read;

pub struct Request {
    pub file_id: String,
}

pub struct Response {
    inner: reqwest::blocking::Response,
}

impl Read for Response {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/v3/reference/files/get
    pub fn get_file(&self, request: Request) -> Result<Response> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}?alt=media",
            request.file_id
        );
        let response = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.reveal()),
            )
            .send()
            .map_err(here!())
            .inspect(|response| println!("[get_file] status {:#?}", response.status()))?;

        let response = Response { inner: response };
        Ok(response)
    }
}

use crate::drive::GoogleDriveClient;
use crate::{here, Result};
use reqwest::Url;
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
    pub fn download_file(&self, request: Request) -> Result<Response> {
        let url = Url::parse_with_params(
            &format!(
                "https://www.googleapis.com/drive/v3/files/{file_id}",
                file_id = request.file_id
            ),
            &[("alt", "media")],
        )
        .map_err(here!())?;

        println!("[get_file] url:{}", url);
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

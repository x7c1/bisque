mod generate_boundary;
use generate_boundary::generate_boundary;

mod reader;
use reader::Reader;

use crate::google_drive_client::GoogleDriveClient;
use crate::Result;
use reqwest::blocking::{Body, RequestBuilder};
use reqwest::IntoUrl;
use std::io::Read;

#[derive(Debug, serde::Serialize)]
pub struct Metadata {
    pub name: String,
    pub parents: Vec<String>,
}

impl GoogleDriveClient {
    pub(crate) fn post_multipart_related<U: IntoUrl>(
        &self,
        url: U,
        metadata: Metadata,
        read: impl Read + Send + 'static,
    ) -> Result<RequestBuilder> {
        let boundary = generate_boundary();
        let reader = Reader::new(read, metadata, &boundary)?;
        let builder = self
            .client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.reveal()),
            )
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(Body::new(reader));

        Ok(builder)
    }
}

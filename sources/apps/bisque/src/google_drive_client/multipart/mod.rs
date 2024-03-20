mod generate_boundary;
use generate_boundary::generate_boundary;

mod reader;
use reader::Reader;

use crate::google_drive_client::GoogleDriveClient;
use reqwest::blocking::{Body, RequestBuilder};
use reqwest::IntoUrl;
use std::fs::File;

#[derive(Debug, serde::Serialize)]
pub struct Metadata {
    pub name: String,
    pub parents: Vec<String>,
}

impl GoogleDriveClient {
    pub fn post_multipart_related<U: IntoUrl>(
        &self,
        url: U,
        metadata: Metadata,
        file: File,
    ) -> RequestBuilder {
        let boundary = generate_boundary();
        let body = Reader::new(file, metadata, &boundary).unwrap();
        self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(Body::new(body))
    }
}

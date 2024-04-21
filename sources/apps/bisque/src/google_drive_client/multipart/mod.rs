mod encryptor;
use encryptor::Encryptor;

mod generate_boundary;
use generate_boundary::generate_boundary;

mod reader;
use reader::Reader;

use crate::google_drive_client::GoogleDriveClient;
use crate::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use reqwest::blocking::{Body, RequestBuilder};
use reqwest::IntoUrl;
use std::fs::File;

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
        file: File,
    ) -> Result<RequestBuilder> {
        let boundary = generate_boundary();
        let encryptor = Encryptor::new(file, generate_key());
        let reader = Reader::new(encryptor, metadata, &boundary)?;
        let builder = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(Body::new(reader));

        Ok(builder)
    }
}

fn generate_key() -> [u8; 16] {
    let mut seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill(&mut seed);
    let mut key = [0; 16];
    key.copy_from_slice(&seed[..16]);
    key
}

pub mod get_file;
pub mod list_files;
pub mod upload_file;

mod multipart;
use crate::oauth::AccessToken;
use crate::{here, Result};
use std::time::Duration;

pub struct GoogleDriveClient {
    client: reqwest::blocking::Client,
    access_token: AccessToken,
}

impl GoogleDriveClient {
    pub fn new(access_token: AccessToken) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(here!())?;

        Ok(Self {
            client,
            access_token,
        })
    }
}

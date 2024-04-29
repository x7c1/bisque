mod download_file;
pub use download_file::DownloadFileParams;

mod upload_file;
pub use upload_file::UploadFileParams;

mod multipart;
pub use multipart::Metadata;

use crate::oauth_client::AccessToken;
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

use crate::oauth_client::AccessToken;

mod upload_file;
pub use upload_file::UploadFileParams;

pub struct GoogleDriveClient {
    client: reqwest::blocking::Client,
    access_token: AccessToken,
}

impl GoogleDriveClient {
    pub fn new(access_token: AccessToken) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            access_token,
        }
    }
}

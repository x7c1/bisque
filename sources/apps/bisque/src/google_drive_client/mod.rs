use crate::oauth_client::AccessToken;

mod upload_file;
pub use upload_file::UploadFileParams;

pub struct GoogleDriveClient {
    access_token: AccessToken,
}

impl GoogleDriveClient {
    pub fn new(access_token: AccessToken) -> Self {
        Self { access_token }
    }
}

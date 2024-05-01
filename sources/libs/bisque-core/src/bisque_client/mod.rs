pub mod command;

pub use crate::{here, Result};
use bisque_google_drive::drive::GoogleDriveClient;
use bisque_google_drive::oauth::AccessToken;

pub struct BisqueClient {
    drive_client: GoogleDriveClient,
}

impl BisqueClient {
    pub fn new(access_token: AccessToken) -> Result<Self> {
        let drive_client = GoogleDriveClient::new(access_token).map_err(here!())?;
        Ok(Self { drive_client })
    }
}

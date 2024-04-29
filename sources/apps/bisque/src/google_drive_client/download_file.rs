use crate::google_drive_client::GoogleDriveClient;
use crate::{here, Result};
use reqwest::Url;
use std::path::PathBuf;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-downloads
    /// https://developers.google.com/drive/api/reference/rest/v3/files/get
    pub fn download_file(&self, params: DownloadFileParams) -> Result<()> {
        println!("{:#?}", params);

        let url = "https://www.googleapis.com/drive/v3/files";
        let url = Url::parse_with_params(
            url,
            &[(
                "q",
                format!(
                    "'{}' in parents and trashed = false and name = '{}'",
                    params.src_folder_id, params.src_name
                ),
            )],
        )
        .map_err(here!())?;

        println!("URL: {:#?}", url);

        let response = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.reveal()),
            )
            .send()
            .map_err(here!())
            .inspect(|response| println!("[download_file] status {:#?}", response.status()))?;

        println!(
            "[download_file] Response: {}",
            response.text().map_err(here!())?
        );

        // TODO: download file as blob
        // let url = "https://www.googleapis.com/drive/v3/files/{fileId}";

        Ok(())
    }
}

#[derive(Debug)]
pub struct DownloadFileParams {
    /// key file to encrypt/decrypt
    pub key_file_path: String,
    pub dst_file_path: PathBuf,
    pub src_name: String,
    pub src_folder_id: String,
}

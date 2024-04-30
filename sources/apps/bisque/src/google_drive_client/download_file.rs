use crate::google_drive_client::GoogleDriveClient;
use crate::{here, Result};
use bisque_cipher::Decrypter;
use reqwest::Url;
use std::io::{Read, Write};
use std::path::PathBuf;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-downloads
    /// https://developers.google.com/drive/api/reference/rest/v3/files/list
    pub fn download_file(&self, params: DownloadFileParams) -> Result<()> {
        println!("{:#?}", params);

        let url = "https://www.googleapis.com/drive/v3/files";
        let query = format!(
            "'{}' in parents and trashed = false and name = '{}'",
            // TODO: escape single quotes
            params.src_folder_id,
            params.src_name,
        );
        let url = Url::parse_with_params(url, &[("q", query)]).map_err(here!())?;

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

        let body = response.text().map_err(here!())?;
        println!("[download_file] Response: {body}",);

        let response = serde_json::from_str::<ListFilesResponse>(&body).map_err(here!())?;
        println!("reified: {:#?}", response);

        // TODO: remove panic
        let found = match response.files.as_slice() {
            [] => panic!("no files found."),
            [file] => file,
            _ => panic!("{} files found.", response.files.len()),
        };
        println!("found: {:#?}", found);

        let response = self.get_file(GetFileRequest {
            file_id: found.id.clone(),
        })?;
        // TODO: use secret key
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut reader = Decrypter::new(response.inner, key, iv).map_err(here!())?;
        let mut buffer = vec![0; 4096];
        let mut file = std::fs::File::create(&params.dst_file_path).map_err(here!())?;
        loop {
            let read = reader.read(&mut buffer).map_err(here!())?;
            if read == 0 {
                break;
            }
            file.write_all(&buffer[..read]).map_err(here!())?;
        }
        Ok(())
    }

    /// https://developers.google.com/drive/api/v3/reference/files/get
    pub fn get_file(&self, request: GetFileRequest) -> Result<GetFileResponse> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}?alt=media",
            request.file_id
        );
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

        let response = GetFileResponse { inner: response };
        Ok(response)
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

/// https://developers.google.com/drive/api/v3/reference/files/get
#[derive(Debug)]
pub struct GetFileRequest {
    pub file_id: String,
}

pub struct GetFileResponse {
    inner: reqwest::blocking::Response,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListFilesResponse {
    pub kind: String,
    #[serde(rename = "incompleteSearch")]
    pub incomplete_search: bool,
    pub files: Vec<File>,
}

#[derive(Debug, serde::Deserialize)]
pub struct File {
    pub kind: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

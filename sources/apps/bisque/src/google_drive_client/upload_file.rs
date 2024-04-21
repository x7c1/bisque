use crate::google_drive_client::{GoogleDriveClient, Metadata};
use crate::{here, Result};
use bisque_cipher::{EncryptionKey, Encryptor};
use std::fs::File;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-uploads#http_1
    pub fn upload_file(&self, params: UploadFileParams) -> Result<()> {
        let file = File::open(&params.src_file_path).map_err(here!())?;
        let file_size = file.metadata().map_err(here!())?.len();

        println!("[upload_file] File size: {}", file_size);
        println!("[upload_file] {:#?}", params);

        let metadata = Metadata {
            name: params.dst_name,
            parents: vec![params.dst_folder_id],
        };
        let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
        let encryptor = Encryptor::new(file, &EncryptionKey::generate());
        let response = self
            .post_multipart_related(url, metadata, encryptor)?
            .send()
            .map_err(here!())
            .inspect(|response| {
                println!("[upload_file] Response status: {}", response.status());
            })?;

        println!(
            "[upload_file] Response: {}",
            response.text().map_err(here!())?
        );
        Ok(())
    }
}

#[derive(Debug)]
pub struct UploadFileParams {
    /// key file to encrypt/decrypt
    pub key_file_path: String,
    pub src_file_path: String,
    pub dst_name: String,
    pub dst_folder_id: String,
}

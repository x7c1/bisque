use crate::bisque_client::BisqueClient;
use crate::models::FilePath;
use crate::{here, Result};
use bisque_cipher::{Encrypter, EncryptionKey, Iv};
use bisque_google_drive::drive::upload_file;
use bisque_google_drive::schemas::Metadata;
use std::fs::File;

#[derive(Debug)]
pub struct Params {
    /// key file to encrypt/decrypt
    pub key_file_path: FilePath,
    pub src_file_path: FilePath,
    pub dst_folder_id: String,
}

impl BisqueClient {
    pub fn upload_file(&self, params: Params) -> Result<()> {
        println!("[upload_file] {:#?}", params);

        let file = File::open(&params.src_file_path).map_err(here!())?;
        let file_size = file.metadata().map_err(here!())?.len();
        println!("[upload_file] File size: {}", file_size);

        let metadata = Metadata {
            name: params.src_file_path.file_name,
            parents: vec![params.dst_folder_id],
        };
        let reader = {
            let key = EncryptionKey::restore_from_file(&params.key_file_path).map_err(here!())?;
            let iv = Iv::generate();
            Encrypter::embed_iv(file, key, iv).map_err(here!())?
        };
        let request = upload_file::Request { metadata, reader };
        self.drive_client.upload_file(request).map_err(here!())?;
        Ok(())
    }
}

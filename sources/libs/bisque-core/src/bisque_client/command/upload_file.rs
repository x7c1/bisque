use crate::bisque_client::BisqueClient;
use crate::{here, Result};
use bisque_cipher::{Encrypter, RandomBytes};
use bisque_google_drive::drive::upload_file;
use bisque_google_drive::schemas::Metadata;
use std::fs::File;

#[derive(Debug)]
pub struct Params {
    /// key file to encrypt/decrypt
    pub key_file_path: String,
    pub src_file_path: String,
    pub dst_name: String,
    pub dst_folder_id: String,
}

impl BisqueClient {
    pub fn upload_file(&self, params: Params) -> Result<()> {
        let file = File::open(&params.src_file_path).map_err(here!())?;
        let file_size = file.metadata().map_err(here!())?.len();

        println!("[upload_file] File size: {}", file_size);
        println!("[upload_file] {:#?}", params);

        let metadata = Metadata {
            name: params.dst_name,
            parents: vec![params.dst_folder_id],
        };
        let key = RandomBytes::restore_from_file(&params.key_file_path)
            .map_err(here!())?
            .into_key();

        let iv = RandomBytes::generate().into_iv();
        let encrypter = Encrypter::new(file, &key, &iv).map_err(here!())?;

        self.drive_client
            .upload_file(upload_file::Request {
                metadata,
                reader: encrypter,
            })
            .map_err(here!())?;

        Ok(())
    }
}

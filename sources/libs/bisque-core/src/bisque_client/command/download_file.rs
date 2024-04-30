use crate::bisque_client::BisqueClient;
use crate::command::download_file::Error::{MultipleFiles, NotFound};
use crate::{here, Result};
use bisque_cipher::{Decrypter, EncryptionKey};
use bisque_google_drive::drive::{download_file, list_files};
use bisque_google_drive::schemas::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Params {
    /// key file to encrypt/decrypt
    pub key_file_path: String,
    pub dst_file_path: PathBuf,
    pub src_name: String,
    pub src_folder_id: String,
}

impl BisqueClient {
    pub fn download_file(&self, params: Params) -> Result<()> {
        println!("{:#?}", params);

        let request = list_files::Request {
            folder_id: params.src_folder_id.clone(),
            name: params.src_name.clone(),
        };
        let response = self.drive_client.list_files(request).map_err(here!())?;
        println!("[download_file] response: {:#?}", response);

        let found = require_single_file(response, &params).map_err(here!())?;
        println!("[download_file] found: {:#?}", found);

        let request = download_file::Request {
            file_id: found.id.clone(),
        };
        let response = self.drive_client.download_file(request).map_err(here!())?;
        let key = EncryptionKey::from_file(&params.key_file_path).map_err(here!())?;
        let mut reader = Decrypter::new(response, &key.into_key()).map_err(here!())?;

        let mut file = std::fs::File::create(&params.dst_file_path).map_err(here!())?;
        io::copy(&mut reader, &mut file).map_err(here!())?;

        Ok(())
    }
}

fn require_single_file(
    response: list_files::Response,
    params: &Params,
) -> std::result::Result<File, Error> {
    match response.files.as_slice() {
        [file] => Ok(file.clone()),
        [] => Err(NotFound {
            name: params.src_name.clone(),
            folder_id: params.src_folder_id.clone(),
        }),
        _ => Err(MultipleFiles {
            name: params.src_name.clone(),
            folder_id: params.src_folder_id.clone(),
            files: response.files,
        }),
    }
}

#[derive(Debug)]
pub enum Error {
    NotFound {
        name: String,
        folder_id: String,
    },
    MultipleFiles {
        name: String,
        folder_id: String,
        files: Vec<File>,
    },
}

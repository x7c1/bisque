use crate::bisque_client::BisqueClient;
use crate::{here, Result};
use bisque_cipher::Decrypter;
use bisque_google_drive::drive::{get_file, list_files};
use std::io::{Read, Write};
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

        let response = self
            .drive_client
            .list_files(list_files::Request {
                folder_id: params.src_folder_id.clone(),
                name: params.src_name.clone(),
            })
            .map_err(here!())?;

        // TODO: remove panic
        let found = match response.files.as_slice() {
            [] => panic!("no files found."),
            [file] => file,
            _ => panic!("{} files found.", response.files.len()),
        };
        println!("found: {:#?}", found);

        let response = self
            .drive_client
            .get_file(get_file::Request {
                file_id: found.id.clone(),
            })
            .map_err(here!())?;

        // TODO: use secret key
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut reader = Decrypter::new(response, key, iv).map_err(here!())?;
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
}

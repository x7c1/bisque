mod reader;
use reader::Reader;

use crate::google_drive_client::GoogleDriveClient;
use crate::Result;
use reqwest::blocking::Body;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::fs::File;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-uploads#http_1
    pub fn upload_file(&self, params: UploadFileParams) -> Result<()> {
        let file = File::open(&params.src_file_path).expect("cannot open");
        let file_size = file.metadata()?.len();
        println!("[upload_file] File size: {}", file_size);
        println!("[upload_file] {:#?}", params);

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=UTF-8"),
        );
        let metadata = Metadata {
            name: params.dst_name,
            parents: vec![params.dst_folder_id],
        };
        // TODO: change boundary to random string
        let boundary = "boundary";
        let response = self
            .client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(Body::new(Reader::new(file, metadata, boundary)?))
            .send()?;

        println!("[upload_file] Response status: {}", response.status());
        println!("[upload_file] Response: {:?}", response.text()?);

        Ok(())
    }
}

#[derive(Debug)]
pub struct UploadFileParams {
    pub src_file_path: String,
    pub dst_name: String,
    pub dst_folder_id: String,
}

#[derive(Debug, serde::Serialize)]
struct Metadata {
    name: String,
    parents: Vec<String>,
}

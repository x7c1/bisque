use crate::google_drive_client::GoogleDriveClient;
use crate::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::fs::File;
use std::io::Read;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-uploads#http_1
    pub fn upload_file(&self, params: UploadFileParams) -> Result<()> {
        let mut file = File::open(&params.src_file_path).expect("cannot open");
        let file_size = file.metadata().unwrap().len();
        println!("[upload_file] File size: {}", file_size);
        println!("[upload_file] {:#?}", params);

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=UTF-8"),
        );

        let client = reqwest::blocking::Client::new();

        let metadata = Metadata {
            name: params.dst_name,
            parents: vec![params.dst_folder_id],
        };

        let response = client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Content-Type",
                // TODO: change boundary to random string
                format!("multipart/related; boundary={}", "boundary"),
            )
            .body({
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).expect("cannot read");

                let body = format!(
                    "--boundary\r\n\
Content-Type: application/json; charset=UTF-8\r\n\r\n\
{}\r\n\
--boundary\r\n\
Content-Type: application/octet-stream\r\n\r\n\
{}\r\n\
--boundary--",
                    serde_json::to_string(&metadata).unwrap(),
                    // TODO: read file in chunks
                    base64::encode(&contents)
                );
                body
            })
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

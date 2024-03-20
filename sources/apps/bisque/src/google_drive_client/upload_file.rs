use crate::google_drive_client::{GoogleDriveClient, Metadata};
use crate::Result;
use std::fs::File;

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-uploads#http_1
    pub fn upload_file(&self, params: UploadFileParams) -> Result<()> {
        let file = File::open(&params.src_file_path)?;
        let file_size = file.metadata()?.len();
        println!("[upload_file] File size: {}", file_size);
        println!("[upload_file] {:#?}", params);

        let metadata = Metadata {
            name: params.dst_name,
            parents: vec![params.dst_folder_id],
        };
        let response = self
            .post_multipart_related(
                "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart",
                metadata,
                file,
            )
            .send()?;

        println!("[upload_file] Response status: {}", response.status());
        println!("[upload_file] Response: {}", response.text()?);
        Ok(())
    }
}

#[derive(Debug)]
pub struct UploadFileParams {
    pub src_file_path: String,
    pub dst_name: String,
    pub dst_folder_id: String,
}

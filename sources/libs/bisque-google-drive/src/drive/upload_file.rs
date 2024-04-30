use crate::drive::GoogleDriveClient;
use crate::schemas::Metadata;
use crate::{here, Result};
use std::io::Read;

pub struct Request<A> {
    pub metadata: Metadata,
    pub reader: A,
}

pub struct Response {}

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/guides/manage-uploads#http_1
    pub fn upload_file<A>(&self, request: Request<A>) -> Result<()>
    where
        A: Read + Send + 'static,
    {
        println!("[upload_file]");

        let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
        let response = self
            .post_multipart_related(url, request.metadata, request.reader)?
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

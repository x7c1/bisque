use crate::drive::GoogleDriveClient;
use crate::schemas::File;
use crate::{here, Result};
use reqwest::Url;

pub struct Request {
    pub folder_id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub kind: String,
    #[serde(rename = "incompleteSearch")]
    pub incomplete_search: bool,
    pub files: Vec<File>,
}

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/reference/rest/v3/files/list
    pub fn list_files(&self, request: Request) -> Result<Response> {
        let url = Url::parse_with_params(
            "https://www.googleapis.com/drive/v3/files",
            &[(
                "q",
                format!(
                    "'{}' in parents and trashed = false and name = '{}'",
                    // TODO: escape single quotes
                    request.folder_id,
                    request.name,
                ),
            )],
        )
        .map_err(here!())?;

        println!("[list_files] url:{}", url);
        let response = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.reveal()),
            )
            .send()
            .map_err(here!())?;

        println!("[list_files] status {:#?}", response.status());
        let body = response.text().map_err(here!())?;
        let response = serde_json::from_str::<Response>(&body).map_err(here!())?;
        Ok(response)
    }
}

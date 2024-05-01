use crate::drive::GoogleDriveClient;
use crate::schemas::File;
use crate::{here, Result};
use reqwest::Url;

pub struct Request {
    pub folder_id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success(SuccessResponse),
    BadRequest { error: BadRequestResponse },
}

#[derive(Debug, serde::Deserialize)]
pub struct SuccessResponse {
    pub kind: String,
    #[serde(rename = "incompleteSearch")]
    pub incomplete_search: bool,
    pub files: Vec<File>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Error {
    pub domain: String,
    pub reason: String,
    pub message: String,
    pub location: String,
    #[serde(rename = "locationType")]
    pub location_type: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct BadRequestResponse {
    pub code: u32,
    pub message: String,
    pub errors: Vec<Error>,
}

impl GoogleDriveClient {
    /// https://developers.google.com/drive/api/reference/rest/v3/files/list
    pub fn list_files(&self, request: Request) -> Result<SuccessResponse> {
        let url = Url::parse_with_params(
            "https://www.googleapis.com/drive/v3/files",
            &[(
                "q",
                format!(
                    "'{}' in parents and trashed = false and name = '{}'",
                    request.folder_id,
                    request.name.replace('\\', "\\\\").replace('\'', "\\'"),
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
        println!("[list_files] body {}", body);

        let response = serde_json::from_str::<Response>(&body).map_err(here!())?;
        match response {
            Response::Success(response) => Ok(response),
            Response::BadRequest { error } => Err(error).map_err(here!())?,
        }
    }
}

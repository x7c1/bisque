/// https://developers.google.com/drive/api/reference/rest/v3/files
#[derive(Debug, serde::Serialize)]
pub struct Metadata {
    pub name: String,
    pub parents: Vec<String>,
}

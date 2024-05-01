#[derive(Clone, Debug, serde::Deserialize)]
pub struct File {
    pub kind: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

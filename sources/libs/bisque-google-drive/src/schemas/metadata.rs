#[derive(Debug, serde::Serialize)]
pub struct Metadata {
    pub name: String,
    pub parents: Vec<String>,
}

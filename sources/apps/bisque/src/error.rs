pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvVarError(std::env::VarError),
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVarError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

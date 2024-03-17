pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvVar(std::env::VarError),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::EnvVar(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

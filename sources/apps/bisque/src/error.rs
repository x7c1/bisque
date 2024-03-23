pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Env(EnvError),
}

#[derive(Debug)]
pub enum EnvError {
    NotPresent {
        key: String,
    },
    Empty {
        key: String,
    },
    Other {
        key: String,
        cause: std::env::VarError,
    },
}

impl From<EnvError> for Error {
    fn from(e: EnvError) -> Self {
        Error::Env(e)
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

use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    Io(IoError),
    Reqwest(ReqwestError),
    SerdeJson(serde_json::Error),
}

#[derive(Debug)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

#[macro_export]
macro_rules! locate {
    () => {
        $crate::error::Location {
            file: file!(),
            line: line!(),
        }
    };
}

#[macro_export]
macro_rules! here {
    () => {
        $crate::error::attach($crate::locate!())
    };
}

pub fn attach<E>(location: Location) -> impl FnOnce(E) -> Error
where
    Error: From<(E, Location)>,
{
    |cause| (cause, location).into()
}

impl From<(io::Error, Location)> for Error {
    fn from((cause, location): (io::Error, Location)) -> Self {
        Error::Io(IoError { cause, location })
    }
}

impl From<(reqwest::Error, Location)> for Error {
    fn from((cause, location): (reqwest::Error, Location)) -> Self {
        Error::Reqwest(ReqwestError { cause, location })
    }
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

#[derive(Debug)]
pub struct IoError {
    pub cause: io::Error,
    pub location: Location,
}

#[derive(Debug)]
pub struct ReqwestError {
    pub cause: reqwest::Error,
    pub location: Location,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

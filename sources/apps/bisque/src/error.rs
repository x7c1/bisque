use std::fmt::Debug;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    Io(Inherited<io::Error>),
    Reqwest(Inherited<reqwest::Error>),
    SerdeJson(Inherited<serde_json::Error>),
    RefreshAccessToken,
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

#[derive(Debug)]
pub struct Inherited<A: Debug> {
    pub cause: A,
    pub location: Location,
}

pub fn attach<E>(location: Location) -> impl FnOnce(E) -> Error
where
    Inherited<E>: Into<Error>,
    E: Debug,
{
    |cause| Inherited { cause, location }.into()
}

impl From<Inherited<io::Error>> for Error {
    fn from(inherited: Inherited<io::Error>) -> Self {
        Error::Io(inherited)
    }
}

impl From<Inherited<reqwest::Error>> for Error {
    fn from(inherited: Inherited<reqwest::Error>) -> Self {
        Error::Reqwest(inherited)
    }
}

impl From<Inherited<serde_json::Error>> for Error {
    fn from(inherited: Inherited<serde_json::Error>) -> Self {
        Error::SerdeJson(inherited)
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

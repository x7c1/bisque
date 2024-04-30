use std::fmt::Debug;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    RefreshAccessToken,
    Inherited(Box<dyn Debug>),
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
pub struct Located<A: Debug> {
    pub cause: A,
    pub location: Location,
}

pub fn attach<E>(location: Location) -> impl FnOnce(E) -> Error
where
    Located<E>: Into<Error>,
    E: Debug,
{
    |cause| Located { cause, location }.into()
}

impl<A: Debug + 'static> From<Located<A>> for Error {
    fn from(value: Located<A>) -> Self {
        Error::Inherited(Box::new(value))
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

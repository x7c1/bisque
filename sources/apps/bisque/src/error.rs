use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Cipher(bisque_cipher::Error),
    Core(bisque_core::Error),
    GoogleDrive(bisque_google_drive::Error),
}

impl From<bisque_cipher::Error> for Error {
    fn from(e: bisque_cipher::Error) -> Self {
        Error::Cipher(e)
    }
}

impl From<bisque_core::Error> for Error {
    fn from(e: bisque_core::Error) -> Self {
        Error::Core(e)
    }
}

impl From<bisque_google_drive::Error> for Error {
    fn from(e: bisque_google_drive::Error) -> Self {
        Error::GoogleDrive(e)
    }
}

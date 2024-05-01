use openssl::error::ErrorStack;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotCreateEncrypter {
        cause: ErrorStack,
    },
    CannotCreateDecrypter {
        cause: ErrorStack,
    },
    KeyFileAlreadyExists {
        path: String,
    },
    CannotReadEmbeddedIv {
        cause: io::Error,
    },
    CannotReadKeyFile {
        path: PathBuf,
        cause: io::Error,
    },
    CannotWriteKeyFile {
        path: String,
        cause: io::Error,
    },
    WrongSizeKeyFile {
        path: PathBuf,
        expected: usize,
        actual: usize,
    },
}

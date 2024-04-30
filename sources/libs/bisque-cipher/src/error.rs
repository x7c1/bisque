use openssl::error::ErrorStack;
use std::io;

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
    CannotReadIv {
        cause: io::Error,
    },
    CannotReadKeyFile {
        path: String,
        cause: io::Error,
    },
    CannotWriteKeyFile {
        path: String,
        cause: io::Error,
    },
    WrongSizeKeyFile {
        path: String,
        expected: usize,
        actual: usize,
    },
}

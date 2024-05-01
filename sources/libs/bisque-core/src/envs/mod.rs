use crate::error::EnvError::{Empty, NotPresent, Other};
use crate::error::Error::Env;
use crate::Result;
use std::env;
use std::env::VarError;

pub fn find(key: &str) -> Result<Option<String>> {
    match require(key) {
        Ok(value) => Ok(Some(value)),
        Err(Env(NotPresent { .. })) | Err(Env(Empty { .. })) => Ok(None),
        Err(cause) => Err(cause),
    }
}

pub fn require(key: &str) -> Result<String> {
    match env::var(key) {
        Ok(value) if value.is_empty() => Err(Empty {
            key: key.to_string(),
        })?,
        Ok(value) => Ok(value),
        Err(VarError::NotPresent) => Err(NotPresent {
            key: key.to_string(),
        })?,
        Err(cause) => Err(Other {
            key: key.to_string(),
            cause,
        })?,
    }
}

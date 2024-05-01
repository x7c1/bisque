use crate::Error::{CannotReadKeyFile, CannotWriteKeyFile, KeyFileAlreadyExists, WrongSizeKeyFile};
use crate::{seed, Result};
use std::fs;
use std::path::Path;

pub struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    const SIZE: usize = 32;

    pub fn generate() -> Self {
        let seed = seed::generate();
        let mut key = [0; Self::SIZE];
        key.copy_from_slice(&seed[..Self::SIZE]);
        Self(key)
    }

    pub fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self(*bytes)
    }

    pub fn restore_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|cause| CannotReadKeyFile {
            path: path.to_path_buf(),
            cause,
        })?;
        if bytes.len() != Self::SIZE {
            return Err(WrongSizeKeyFile {
                path: path.to_path_buf(),
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }
        let mut array = [0; Self::SIZE];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }

    pub fn write_to_file(&self, path: &str) -> Result<()> {
        let file_exists = fs::metadata(path).is_ok();
        if file_exists {
            return Err(KeyFileAlreadyExists {
                path: path.to_string(),
            });
        }
        fs::write(path, self.0).map_err(|cause| CannotWriteKeyFile {
            path: path.to_string(),
            cause,
        })?;
        Ok(())
    }
}

impl From<&[u8; 32]> for EncryptionKey {
    fn from(bytes: &[u8; 32]) -> Self {
        Self(*bytes)
    }
}

use crate::Error::{CannotReadKeyFile, CannotWriteKeyFile, KeyFileAlreadyExists, WrongSizeKeyFile};
use crate::Result;
use aes::cipher::consts::U32;
use aes::cipher::generic_array::GenericArray;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;

pub struct EncryptionKey([u8; Self::SIZE]);

impl EncryptionKey {
    const SIZE: usize = 32;

    pub fn generate() -> EncryptionKey {
        let seed = Self::generate_seed();
        let mut key = [0; Self::SIZE];
        key.copy_from_slice(&seed[..Self::SIZE]);
        EncryptionKey(key)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let bytes = fs::read(path).map_err(|cause| CannotReadKeyFile {
            path: path.to_string(),
            cause,
        })?;
        if bytes.len() != Self::SIZE {
            return Err(WrongSizeKeyFile {
                path: path.to_string(),
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }
        let mut array = [0; Self::SIZE];
        array.copy_from_slice(&bytes);
        Ok(EncryptionKey(array))
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

    pub fn as_array(&self) -> &GenericArray<u8, U32> {
        GenericArray::from_slice(&self.0)
    }

    fn generate_seed() -> [u8; Self::SIZE * 2] {
        let mut seed = [0; Self::SIZE * 2];
        let mut rng = StdRng::from_entropy();
        rng.fill(&mut seed);
        seed
    }
}

use crate::Result;
use aes::cipher::consts::U32;
use aes::cipher::generic_array::GenericArray;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;

pub struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    pub fn generate() -> EncryptionKey {
        let mut seed: [u8; 64] = [0; 64];
        let mut rng = StdRng::from_entropy();
        rng.fill(&mut seed);

        let mut key = [0; 32];
        key.copy_from_slice(&seed[..32]);
        EncryptionKey(key)
    }
    pub fn from_file(path: String) -> Result<Self> {
        let bytes = fs::read(path).unwrap();
        if bytes.len() != 32 {
            todo!()
        }
        let mut array = [0; 32];
        array.copy_from_slice(&bytes);
        Ok(EncryptionKey(array))
    }
    pub fn as_array(&self) -> &GenericArray<u8, U32> {
        GenericArray::from_slice(&self.0)
    }
}

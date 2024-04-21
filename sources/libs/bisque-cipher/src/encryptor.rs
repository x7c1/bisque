use crate::EncryptionKey;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::Aes256;
use std::io;
use std::io::Read;

const BLOCK_SIZE: usize = 16;

pub struct Encryptor<R> {
    inner: R,
    cipher: Aes256,
}

impl<R: Read> Encryptor<R> {
    pub fn new(reader: R, key: &EncryptionKey) -> Self {
        Encryptor {
            inner: reader,
            cipher: Aes256::new(key.as_array()),
        }
    }
}

impl<R: Read> Read for Encryptor<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut inner_buffer = [0; BLOCK_SIZE];
        let bytes_read = self.inner.read(&mut inner_buffer)?;
        if bytes_read == 0 {
            return Ok(0); // End of inner stream
        }
        let mut chunk = inner_buffer.to_vec();
        let mut position = 0;
        while position < chunk.len() {
            let block = GenericArray::from_mut_slice(&mut chunk[position..position + BLOCK_SIZE]);
            self.cipher.encrypt_block(block);
            position += BLOCK_SIZE;
        }
        let to_copy = std::cmp::min(buf.len(), chunk.len());
        buf[..to_copy].copy_from_slice(&chunk[..to_copy]);

        Ok(to_copy)
    }
}

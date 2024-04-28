use crate::Error::CannotCreateEncryptor;
use crate::Result;
use openssl::symm::{Cipher, Crypter, Mode};
use std::io;
use std::io::{Read, Write};
use std::process::exit;

const BLOCK_SIZE: usize = 16;

pub struct Encryptor<R> {
    inner: R,
    crypter: Crypter,
    block_size: usize,
    finalized: bool,
}

impl<R: Read> Encryptor<R> {
    pub fn new(reader: R, key: &[u8], iv: &[u8]) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();
        let crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))
            .map_err(|cause| CannotCreateEncryptor { cause })?;

        Ok(Encryptor {
            inner: reader,
            crypter,
            block_size: cipher.block_size(),
            finalized: false,
        })
    }

    fn finalize(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.finalized {
            return Ok(0);
        }
        let mut output = vec![0; self.block_size];
        let written = self.crypter.finalize(&mut output)?;
        buf.write_all(&output)?;
        self.finalized = true;
        Ok(written)
    }
}

impl<R: Read> Read for Encryptor<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let mut cache = vec![0; buf.len()];
        let loaded = self.inner.read(&mut cache)?;

        let mut output = vec![0; loaded + self.block_size];
        let written = {
            let input = &cache[..loaded];
            self.crypter.update(input, &mut output)?
        };
        if written == 0 {
            self.finalize(&mut buf)
        } else {
            buf.write_all(&output[..written])?;
            Ok(written)
        }
    }
}

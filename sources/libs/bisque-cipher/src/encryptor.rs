use crate::Error::CannotCreateEncryptor;
use crate::Result;
use openssl::symm::{Cipher, Crypter, Mode};
use std::io;
use std::io::{Read, Write};

pub struct Encryptor<R> {
    inner: R,
    crypter: Crypter,
    block_size: usize,
    finalized: bool,
    buffer: Vec<u8>,
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
            buffer: vec![],
        })
    }

    fn finalize(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.finalized {
            return Ok(0);
        }
        let mut output = vec![0; self.block_size];
        let written = self.crypter.finalize(&mut output)?;
        buf.write(&output[..written])?;
        self.finalized = true;
        Ok(written)
    }
}

impl<R: Read> Read for Encryptor<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if self.buffer.len() > 0 {
            let (moved, remaining) = move_buffer(&mut buf, &self.buffer)?;
            self.buffer = remaining.to_vec();
            return Ok(moved);
        }
        let mut cache = vec![0; 4192];
        let loaded = self.inner.read(&mut cache)?;

        let mut output = vec![0; loaded + self.block_size];
        let updated = {
            let input = &cache[..loaded];
            self.crypter.update(input, &mut output)?
        };
        if updated == 0 {
            self.finalize(buf)
        } else {
            let (moved, remaining) = move_buffer(&mut buf, &output[..updated])?;
            self.buffer = remaining.to_vec();
            Ok(moved)
        }
    }
}

fn move_buffer(mut dst: &mut [u8], src: &[u8]) -> io::Result<(usize, Vec<u8>)> {
    let buffer_loaded = dst.len().min(src.len());
    let (bytes, remaining) = src.split_at(buffer_loaded);
    dst.write(bytes)?;
    Ok((buffer_loaded, remaining.to_vec()))
}

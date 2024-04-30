use crate::Error::CannotCreateDecrypter;
use crate::Result;
use openssl::symm::{Cipher, Crypter, Mode};
use std::io;
use std::io::{Read, Write};

pub struct Decrypter<R> {
    inner: R,
    crypter: Crypter,
    block_size: usize,
    finalized: bool,
    buffer: Vec<u8>,
    buffer_min_size: usize,
}

impl<R: Read> Decrypter<R> {
    pub fn new(reader: R, key: &[u8], iv: &[u8]) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();
        let crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))
            .map_err(|cause| CannotCreateDecrypter { cause })?;

        Ok(Decrypter {
            inner: reader,
            crypter,
            block_size: cipher.block_size(),
            finalized: false,
            buffer: vec![],
            buffer_min_size: 4096,
        })
    }

    fn finalize(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.finalized {
            return Ok(0);
        }
        let mut output = vec![0; self.block_size];
        let written = self.crypter.finalize(&mut output)?;
        self.finalized = true;

        let (moved, remaining) = move_buffer(buf, &output[..written])?;
        self.buffer = remaining.to_vec();
        Ok(moved)
    }
}

impl<R: Read> Read for Decrypter<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.buffer.is_empty() {
            let (moved, remaining) = move_buffer(buf, &self.buffer)?;
            self.buffer = remaining.to_vec();
            return Ok(moved);
        }
        let mut buffer = vec![0; self.buffer_min_size.max(buf.len())];
        let loaded = self.inner.read(&mut buffer)?;

        let mut output = vec![0; loaded + self.block_size];
        let written = {
            let input = &buffer[..loaded];
            self.crypter.update(input, &mut output)?
        };
        if written == 0 {
            self.finalize(buf)
        } else {
            let (moved, remaining) = move_buffer(buf, &output[..written])?;
            self.buffer = remaining.to_vec();
            Ok(moved)
        }
    }
}

fn move_buffer<'a>(mut dst: &'a mut [u8], src: &'a [u8]) -> io::Result<(usize, &'a [u8])> {
    let moved = dst.len().min(src.len());
    let (bytes, remaining) = src.split_at(moved);
    dst.write_all(bytes)?;
    Ok((moved, remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openssl_usage;
    use rstest::rstest;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    #[rstest(input_file, decrypted_file, expected_file)]
    #[case::empty_text(
        "./samples/input_encrypted_empty.cbc",
        "./samples/test4_0_decrypted_output1.txt.tmp",
        "./samples/input_empty.txt"
    )]
    #[case::small_text(
        "./samples/input_encrypted_smaller_than_block_size.cbc",
        "./samples/test4_1_decrypted_output1.txt.tmp",
        "./samples/input_smaller_than_block_size.txt"
    )]
    #[case::large_text(
        "./samples/input_encrypted_larger_than_block_size.cbc",
        "./samples/test4_2_decrypted_output1.cbc.tmp",
        "./samples/input_larger_than_block_size.txt"
    )]
    #[case::image(
        "./samples/input_encrypted_image.cbc",
        "./samples/test4_3_decrypted_output1.png.tmp",
        "./samples/input_image.png"
    )]
    fn test4_decrypter(input_file: &str, decrypted_file: &str, expected_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut decrypter = Decrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();

        let mut file = File::create(decrypted_file).unwrap();
        let mut bytes = vec![];
        let _len = decrypter.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();

        // openssl_usage::decrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(decrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }
}

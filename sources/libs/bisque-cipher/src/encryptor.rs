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
    buffer_min_size: usize,
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

impl<R: Read> Read for Encryptor<R> {
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

    #[rstest(input_file, encrypted_file, expected_file)]
    #[case::small_text(
        "./samples/input_empty.txt",
        "./samples/test2_0_encrypted_output1.cbc.tmp",
        "./samples/test2_0_expected_output1.cbc.tmp"
    )]
    #[case::small_text(
        "./samples/input_smaller_than_block_size.txt",
        "./samples/test2_1_encrypted_output1.cbc.tmp",
        "./samples/test2_1_expected_output1.cbc.tmp"
    )]
    #[case::large_text(
        "./samples/input_larger_than_block_size.txt",
        "./samples/test2_2_encrypted_output1.cbc.tmp",
        "./samples/test2_2_expected_output1.cbc.tmp"
    )]
    #[case::image(
        "./samples/input_image.png",
        "./samples/test2_3_encrypted_output1.cbc.tmp",
        "./samples/test2_3_expected_output1.png.tmp"
    )]
    fn test2_encryptor(input_file: &str, encrypted_file: &str, expected_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut encryptor = Encryptor::new(File::open(input_file).unwrap(), key, iv).unwrap();

        let mut file = File::create(encrypted_file).unwrap();
        let mut bytes = vec![];
        let _len = encryptor.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();

        openssl_usage::encrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(encrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }

    #[rstest]
    fn test3_uneven_read_call() {
        let input_file = "./samples/input_image.png";
        let output_file = "./samples/test3_1_encrypted_output.png";
        let expected_file = "./samples/test3_1_expected_output.png";

        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut encryptor = Encryptor::new(File::open(input_file).unwrap(), key, iv).unwrap();

        // Encryptor::read() was called with these byte counts
        // by reqwest post() although the reason was unclear.
        let uneven_bytes = &[
            7883, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 6955, 6939,
        ];
        let mut file = File::create(output_file).unwrap();
        for bytes in uneven_bytes {
            let mut buffer = vec![0; *bytes];
            let loaded = encryptor.read(&mut buffer).unwrap();
            let _written = file.write(&buffer[..loaded]).unwrap();
        }
        openssl_usage::encrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(output_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }
}

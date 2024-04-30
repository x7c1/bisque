use crate::openssl_impls::Crypter;
use crate::Error::CannotCreateEncrypter;
use crate::Result;
use openssl::symm;
use openssl::symm::{Cipher, Mode};
use std::io;
use std::io::Read;

pub struct Encrypter<R> {
    inner: Crypter<R>,
}

impl<R: Read> Encrypter<R> {
    pub fn new(reader: R, key: &[u8], iv: &[u8]) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();
        let openssl_crypter = symm::Crypter::new(cipher, Mode::Encrypt, key, Some(iv))
            .map_err(|cause| CannotCreateEncrypter { cause })?;

        Ok(Encrypter {
            inner: Crypter::new(reader, openssl_crypter, cipher.block_size())?,
        })
    }
}

impl<R: Read> Read for Encrypter<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
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
    #[case::empty_text(
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
    fn test2_encrypter(input_file: &str, encrypted_file: &str, expected_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut encrypter = Encrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();

        let mut file = File::create(encrypted_file).unwrap();
        let mut bytes = vec![];
        let _len = encrypter.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();

        openssl_usage::encrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(encrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }

    #[rstest]
    fn test3_uneven_read_call() {
        let input_file = "./samples/input_image.png";
        let output_file = "./samples/test3_1_encrypted_output.png.tmp";
        let expected_file = "./samples/test3_1_expected_output.png.tmp";

        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut encrypter = Encrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();

        // Encrypter::read() was called with these byte counts
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
            let loaded = encrypter.read(&mut buffer).unwrap();
            let _written = file.write(&buffer[..loaded]).unwrap();
        }
        openssl_usage::encrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(output_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }
}
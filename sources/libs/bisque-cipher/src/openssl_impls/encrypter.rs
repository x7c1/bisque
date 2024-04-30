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
    use rstest::rstest;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    #[rstest(input_file, encrypted_file, expected_file)]
    #[case::empty_text(
        "./samples/decrypted/empty.txt",
        "./samples/encrypted.output/test2_0.cbc",
        "./samples/encrypted/empty.cbc"
    )]
    #[case::small_text(
        "./samples/decrypted/text_smaller_than_block_size.txt",
        "./samples/encrypted.output/test2_1.cbc",
        "./samples/encrypted/text_smaller_than_block_size.cbc"
    )]
    #[case::large_text(
        "./samples/decrypted/text_larger_than_block_size.txt",
        "./samples/encrypted.output/test2_2.cbc",
        "./samples/encrypted/text_larger_than_block_size.cbc"
    )]
    #[case::image(
        "./samples/decrypted/image.png",
        "./samples/encrypted.output/test2_3.cbc",
        "./samples/encrypted/image.cbc"
    )]
    fn test2_encrypter(input_file: &str, encrypted_file: &str, expected_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        let mut encrypter = Encrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();

        let mut file = File::create(encrypted_file).unwrap();
        let mut bytes = vec![];
        let _len = encrypter.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();

        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(encrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }

    #[rstest]
    fn test3_uneven_read_call() {
        let input_file = "./samples/decrypted/image.png";
        let output_file = "./samples/encrypted.output/test3_1.cbc";
        let expected_file = "./samples/encrypted/image.cbc";

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
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(output_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }
}

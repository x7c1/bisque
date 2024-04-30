use crate::openssl_impls::Crypter;
use crate::Error::{CannotCreateDecrypter, CannotReadIv};
use crate::Result;
use openssl::symm;
use openssl::symm::{Cipher, Mode};
use std::io;
use std::io::Read;

pub struct Decrypter<R> {
    inner: Crypter<R>,
}

impl<R: Read> Decrypter<R> {
    pub fn new(mut reader: R, key: &[u8]) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();

        // extract iv bytes from head of reader
        let mut iv = [0; 16];
        reader
            .read_exact(&mut iv)
            .map_err(|cause| CannotReadIv { cause })?;

        let openssl_crypter = symm::Crypter::new(cipher, Mode::Decrypt, key, Some(&iv))
            .map_err(|cause| CannotCreateDecrypter { cause })?;

        Ok(Decrypter {
            inner: Crypter::new(reader, openssl_crypter, cipher.block_size(), vec![])?,
        })
    }
}

impl<R: Read> Read for Decrypter<R> {
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

    #[rstest(input_file, decrypted_file, expected_file)]
    #[case::empty_text(
        "./samples/encrypted/empty_v2.cbc",
        "./samples/decrypted.output/test4_0.txt",
        "./samples/decrypted/empty.txt"
    )]
    #[case::small_text(
        "./samples/encrypted/text_smaller_than_block_size_v2.cbc",
        "./samples/decrypted.output/test4_1.txt",
        "./samples/decrypted/text_smaller_than_block_size.txt"
    )]
    #[case::large_text(
        "./samples/encrypted/text_larger_than_block_size_v2.cbc",
        "./samples/decrypted.output/test4_2.txt",
        "./samples/decrypted/text_larger_than_block_size.txt"
    )]
    #[case::image(
        "./samples/encrypted/image_v2.cbc",
        "./samples/decrypted.output/test4_3.png",
        "./samples/decrypted/image.png"
    )]
    fn test4_decrypter(input_file: &str, decrypted_file: &str, expected_file: &str) {
        let key = b"01234567890123456789012345678901";
        let mut decrypter = Decrypter::new(File::open(input_file).unwrap(), key).unwrap();

        let mut file = File::create(decrypted_file).unwrap();
        let mut bytes = vec![];
        let _len = decrypter.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();

        // openssl_usage::decrypt_file(input_file, expected_file, key, iv).unwrap();
        let expected_bytes = fs::read(expected_file).unwrap();
        let actual_bytes = fs::read(decrypted_file).unwrap();
        assert_eq!(actual_bytes, expected_bytes);
    }
}

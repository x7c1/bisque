use crate::openssl_impls::Crypter;
use crate::Error::CannotCreateDecrypter;
use crate::Result;
use openssl::symm;
use openssl::symm::{Cipher, Mode};
use std::io;
use std::io::Read;

pub struct Decrypter<R> {
    inner: Crypter<R>,
}

impl<R: Read> Decrypter<R> {
    pub fn new(reader: R, key: &[u8], iv: &[u8]) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();
        let openssl_crypter = symm::Crypter::new(cipher, Mode::Decrypt, key, Some(iv))
            .map_err(|cause| CannotCreateDecrypter { cause })?;

        Ok(Decrypter {
            inner: Crypter::new(reader, openssl_crypter, cipher.block_size())?,
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
        "./samples/encrypted/empty.cbc",
        "./samples/decrypted.output/test4_0.txt",
        "./samples/decrypted/empty.txt"
    )]
    #[case::small_text(
        "./samples/encrypted/text_smaller_than_block_size.cbc",
        "./samples/decrypted.output/test4_1.txt",
        "./samples/decrypted/text_smaller_than_block_size.txt"
    )]
    #[case::large_text(
        "./samples/encrypted/text_larger_than_block_size.cbc",
        "./samples/decrypted.output/test4_2.txt",
        "./samples/decrypted/text_larger_than_block_size.txt"
    )]
    #[case::image(
        "./samples/encrypted/image.cbc",
        "./samples/decrypted.output/test4_3.png",
        "./samples/decrypted/image.png"
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

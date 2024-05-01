use crate::openssl_impls::Crypter;
use crate::Error::{CannotCreateDecrypter, CannotReadEmbeddedIv};
use crate::{EncryptionKey, Iv, Result};
use openssl::symm;
use openssl::symm::{Cipher, Mode};
use std::io;
use std::io::Read;

pub struct Decrypter<R> {
    inner: Crypter<R>,
}

impl<R: Read> Decrypter<R> {
    pub fn new(reader: R, key: impl Into<EncryptionKey>, iv: impl Into<Iv>) -> Result<Self> {
        Self::create(reader, key, iv, vec![])
    }

    pub fn extract_iv(mut reader: R, key: impl Into<EncryptionKey>) -> Result<Self> {
        let iv = Self::extract_iv_from_header(&mut reader)?;
        Self::create(reader, key, &iv, vec![])
    }

    fn create(
        reader: R,
        key: impl Into<EncryptionKey>,
        iv: impl Into<Iv>,
        embedded: Vec<u8>,
    ) -> Result<Self> {
        let cipher = Cipher::aes_256_cbc();
        let crypter = symm::Crypter::new(
            cipher,
            Mode::Decrypt,
            key.into().as_bytes(),
            Some(iv.into().as_bytes()),
        )
        .map_err(|cause| CannotCreateDecrypter { cause })?;

        Ok(Self {
            inner: Crypter::new(reader, crypter, cipher.block_size(), embedded)?,
        })
    }

    fn extract_iv_from_header(reader: &mut R) -> Result<[u8; 16]> {
        let mut iv = [0; 16];
        reader
            .read_exact(&mut iv)
            .map_err(|cause| CannotReadEmbeddedIv { cause })?;

        Ok(iv)
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
    use crate::openssl_impls::helper::write_file;
    use rstest::rstest;
    use std::fs;
    use std::fs::File;

    mod test_new {
        use super::*;

        #[rstest(input_file, decrypted_file, expected_file)]
        #[case::empty_text(
            "./samples/encrypted/empty.cbc",
            "./samples/decrypted.output/test7_0_empty.txt",
            "./samples/decrypted/empty.txt"
        )]
        #[case::small_text(
            "./samples/encrypted/text_smaller_than_block_size.cbc",
            "./samples/decrypted.output/test7_1_smaller.txt",
            "./samples/decrypted/text_smaller_than_block_size.txt"
        )]
        #[case::large_text(
            "./samples/encrypted/text_larger_than_block_size.cbc",
            "./samples/decrypted.output/test7_2_larger.txt",
            "./samples/decrypted/text_larger_than_block_size.txt"
        )]
        #[case::image(
            "./samples/encrypted/image.cbc",
            "./samples/decrypted.output/test7_3_image.png",
            "./samples/decrypted/image.png"
        )]
        fn test7(input_file: &str, decrypted_file: &str, expected_file: &str) {
            let key = b"01234567890123456789012345678901";
            let iv = b"0123456789012345";
            let file = File::open(input_file).unwrap();
            let mut decrypter = Decrypter::new(file, key, iv).unwrap();
            write_file(decrypted_file, &mut decrypter);

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(decrypted_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }
    }

    mod test_extract_iv {
        use super::*;

        #[rstest(input_file, decrypted_file, expected_file)]
        #[case::empty_text(
            "./samples/encrypted/empty_v2.cbc",
            "./samples/decrypted.output/test4_0_empty.txt",
            "./samples/decrypted/empty.txt"
        )]
        #[case::small_text(
            "./samples/encrypted/text_smaller_than_block_size_v2.cbc",
            "./samples/decrypted.output/test4_1_smaller.txt",
            "./samples/decrypted/text_smaller_than_block_size.txt"
        )]
        #[case::large_text(
            "./samples/encrypted/text_larger_than_block_size_v2.cbc",
            "./samples/decrypted.output/test4_2_larger.txt",
            "./samples/decrypted/text_larger_than_block_size.txt"
        )]
        #[case::image(
            "./samples/encrypted/image_v2.cbc",
            "./samples/decrypted.output/test4_3_image.png",
            "./samples/decrypted/image.png"
        )]
        fn test4(input_file: &str, decrypted_file: &str, expected_file: &str) {
            let key = b"01234567890123456789012345678901";
            let file = File::open(input_file).unwrap();
            let mut decrypter = Decrypter::extract_iv(file, key).unwrap();
            write_file(decrypted_file, &mut decrypter);

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(decrypted_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }
    }
}

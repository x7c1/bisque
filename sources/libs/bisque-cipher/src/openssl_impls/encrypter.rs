use crate::openssl_impls::Crypter;
use crate::Error::CannotCreateEncrypter;
use crate::{EncryptionKey, Iv, Result};
use openssl::symm;
use openssl::symm::{Cipher, Mode};
use std::io;
use std::io::Read;

pub struct Encrypter<R> {
    inner: Crypter<R>,
}

impl<R: Read> Encrypter<R> {
    pub fn new(reader: R, key: impl Into<EncryptionKey>, iv: impl Into<Iv>) -> Result<Self> {
        Self::create(reader, key, iv, vec![])
    }

    pub fn embed_iv(reader: R, key: impl Into<EncryptionKey>, iv: impl Into<Iv>) -> Result<Self> {
        let iv = iv.into();
        let embedded = iv.as_bytes().to_vec();
        Self::create(reader, key, iv, embedded)
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
            Mode::Encrypt,
            key.into().as_bytes(),
            Some(iv.into().as_bytes()),
        )
        .map_err(|cause| CannotCreateEncrypter { cause })?;

        Ok(Self {
            inner: Crypter::new(reader, crypter, cipher.block_size(), embedded)?,
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
    use crate::openssl_impls::helper::{write_file, write_file_in_chunks};
    use rstest::rstest;
    use std::fs;
    use std::fs::File;

    mod test_new {
        use super::*;

        #[rstest(input_file, encrypted_file, expected_file)]
        #[case::empty_text(
            "./samples/decrypted/empty.txt",
            "./samples/encrypted.output/test5_0_empty.cbc",
            "./samples/encrypted/empty.cbc"
        )]
        #[case::small_text(
            "./samples/decrypted/text_smaller_than_block_size.txt",
            "./samples/encrypted.output/test5_1_smaller.cbc",
            "./samples/encrypted/text_smaller_than_block_size.cbc"
        )]
        #[case::large_text(
            "./samples/decrypted/text_larger_than_block_size.txt",
            "./samples/encrypted.output/test5_2_larger.cbc",
            "./samples/encrypted/text_larger_than_block_size.cbc"
        )]
        #[case::image(
            "./samples/decrypted/image.png",
            "./samples/encrypted.output/test5_3_image.cbc",
            "./samples/encrypted/image.cbc"
        )]
        fn test5(input_file: &str, encrypted_file: &str, expected_file: &str) {
            let key = b"01234567890123456789012345678901";
            let iv = b"0123456789012345";
            let mut encrypter = Encrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();
            write_file(encrypted_file, &mut encrypter);

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(encrypted_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }

        #[rstest]
        fn test6_uneven_read_call() {
            let input_file = "./samples/decrypted/image.png";
            let output_file = "./samples/encrypted.output/test6_1_uneven.cbc";
            let expected_file = "./samples/encrypted/image.cbc";

            let key = b"01234567890123456789012345678901";
            let iv = b"0123456789012345";
            let mut encrypter = Encrypter::new(File::open(input_file).unwrap(), key, iv).unwrap();
            write_file_in_chunks(output_file, &mut encrypter, create_uneven_bytes());

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(output_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }
    }

    mod test_embed_iv {
        use super::*;

        #[rstest(input_file, encrypted_file, expected_file)]
        #[case::empty_text(
            "./samples/decrypted/empty.txt",
            "./samples/encrypted.output/test2_0_empty.cbc",
            "./samples/encrypted/empty_v2.cbc"
        )]
        #[case::small_text(
            "./samples/decrypted/text_smaller_than_block_size.txt",
            "./samples/encrypted.output/test2_1_smaller.cbc",
            "./samples/encrypted/text_smaller_than_block_size_v2.cbc"
        )]
        #[case::large_text(
            "./samples/decrypted/text_larger_than_block_size.txt",
            "./samples/encrypted.output/test2_2_larger.cbc",
            "./samples/encrypted/text_larger_than_block_size_v2.cbc"
        )]
        #[case::image(
            "./samples/decrypted/image.png",
            "./samples/encrypted.output/test2_3_image.cbc",
            "./samples/encrypted/image_v2.cbc"
        )]
        fn test2(input_file: &str, encrypted_file: &str, expected_file: &str) {
            let key = b"01234567890123456789012345678901";
            let iv = b"0123456789012345";
            let mut encrypter =
                Encrypter::embed_iv(File::open(input_file).unwrap(), key, iv).unwrap();

            write_file(encrypted_file, &mut encrypter);

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(encrypted_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }

        #[rstest]
        fn test3_uneven_read_call() {
            let input_file = "./samples/decrypted/image.png";
            let output_file = "./samples/encrypted.output/test3_1_uneven.cbc";
            let expected_file = "./samples/encrypted/image_v2.cbc";

            let key = b"01234567890123456789012345678901";
            let iv = b"0123456789012345";
            let mut encrypter =
                Encrypter::embed_iv(File::open(input_file).unwrap(), key, iv).unwrap();

            write_file_in_chunks(output_file, &mut encrypter, create_uneven_bytes());

            let expected_bytes = fs::read(expected_file).unwrap();
            let actual_bytes = fs::read(output_file).unwrap();
            assert_eq!(actual_bytes, expected_bytes);
        }
    }

    /// Encrypter::read() was called with these byte counts
    /// by reqwest post() although the reason was unclear.
    fn create_uneven_bytes() -> Vec<usize> {
        vec![
            7883, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 8192, 8187, 11, //
            8192, 8187, 6955, 6939,
        ]
    }
}

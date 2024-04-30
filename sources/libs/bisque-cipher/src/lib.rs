mod openssl_usage;

mod decryptor;

mod encryption_key;
pub use encryption_key::EncryptionKey;

mod encryptor;
pub use encryptor::Encryptor;

mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::fs;

    #[rstest(input_file, encrypted_file, decrypted_file)]
    #[case::small_text(
        "./samples/input_smaller_than_block_size.txt",
        "./samples/test1_1_encrypted_output1.cbc.tmp",
        "./samples/test1_1_decrypted_output1.txt.tmp"
    )]
    #[case::large_text(
        "./samples/input_larger_than_block_size.txt",
        "./samples/test1_2_encrypted_output1.cbc.tmp",
        "./samples/test1_2_decrypted_output1.txt.tmp"
    )]
    #[case::image(
        "./samples/input_image.png",
        "./samples/test1_3_encrypted_output2.cbc.tmp",
        "./samples/test1_3_decrypted_output2.png.tmp"
    )]
    fn test1_crypter_of_openssl(input_file: &str, encrypted_file: &str, decrypted_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        openssl_usage::encrypt_file(input_file, encrypted_file, key, iv).unwrap();
        openssl_usage::decrypt_file(encrypted_file, decrypted_file, key, iv).unwrap();

        let expected_bytes = fs::read(input_file).unwrap();
        let actual_bytes = fs::read(decrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }
}

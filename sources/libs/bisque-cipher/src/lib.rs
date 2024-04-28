mod openssl_usage;

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
    use std::fs::File;
    use std::io::{Read, Write};

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
    fn test_crypter_of_openssl(input_file: &str, encrypted_file: &str, decrypted_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        openssl_usage::encrypt_file(input_file, encrypted_file, key, iv).unwrap();
        openssl_usage::decrypt_file(encrypted_file, decrypted_file, key, iv).unwrap();

        let expected_bytes = fs::read(input_file).unwrap();
        let actual_bytes = fs::read(decrypted_file).unwrap();
        assert_eq!(expected_bytes, actual_bytes);
    }

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
        "./samples/test2_3_encrypted_output2.cbc.tmp",
        "./samples/test2_3_expected_output2.png.tmp"
    )]
    fn test_encryptor(input_file: &str, encrypted_file: &str, expected_file: &str) {
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
    fn test_uneven_read_call() {
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

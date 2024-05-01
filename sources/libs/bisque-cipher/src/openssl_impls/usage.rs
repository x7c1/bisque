use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[allow(dead_code)]
pub fn encrypt_file(
    input_file: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    key: &[u8],
    iv: &[u8],
) -> io::Result<()> {
    run(Mode::Encrypt, input_file, output_file, key, iv)
}

#[allow(dead_code)]
pub fn decrypt_file(
    input_file: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    key: &[u8],
    iv: &[u8],
) -> io::Result<()> {
    run(Mode::Decrypt, input_file, output_file, key, iv)
}

fn run(
    mode: Mode,
    input_file: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    key: &[u8],
    iv: &[u8],
) -> io::Result<()> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, mode, key, Some(iv))?;

    let mut input_file = File::open(input_file)?;
    let mut output_file = File::create(output_file)?;

    let mut buffer = [0; 4096];
    let mut total_bytes_written = 0;
    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let input = &buffer[..bytes_read];
        let mut output = vec![0; bytes_read + cipher.block_size()];
        let bytes_written = crypter.update(input, &mut output)?;
        output_file.write_all(&output[..bytes_written])?;
        total_bytes_written += bytes_written;
    }
    let mut output = vec![0; cipher.block_size()];
    total_bytes_written += crypter.finalize(&mut output)?;
    output_file.write_all(&output)?;
    output_file.set_len(total_bytes_written as u64)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::openssl_impls::usage;
    use rstest::rstest;
    use std::fs;

    #[rstest(input_file, encrypted_file, decrypted_file)]
    #[case::small_text(
        "./samples/decrypted/text_smaller_than_block_size.txt",
        "./samples/encrypted.output/test1_1_smaller.cbc",
        "./samples/decrypted.output/test1_1_smaller.txt"
    )]
    #[case::large_text(
        "./samples/decrypted/text_larger_than_block_size.txt",
        "./samples/encrypted.output/test1_2_larger.cbc",
        "./samples/decrypted.output/test1_2_larger.txt"
    )]
    #[case::image(
        "./samples/decrypted/image.png",
        "./samples/encrypted.output/test1_3_image.cbc",
        "./samples/decrypted.output/test1_3_image.png"
    )]
    fn test1_crypter_of_openssl(input_file: &str, encrypted_file: &str, decrypted_file: &str) {
        let key = b"01234567890123456789012345678901";
        let iv = b"0123456789012345";
        usage::encrypt_file(input_file, encrypted_file, key, iv).unwrap();
        usage::decrypt_file(encrypted_file, decrypted_file, key, iv).unwrap();

        let expected_bytes = fs::read(input_file).unwrap();
        let actual_bytes = fs::read(decrypted_file).unwrap();
        assert_eq!(actual_bytes, expected_bytes);
    }
}

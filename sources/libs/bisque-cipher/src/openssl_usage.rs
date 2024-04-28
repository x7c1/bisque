use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn encrypt_file(
    input_file: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    key: &[u8],
    iv: &[u8],
) -> io::Result<()> {
    run(Mode::Encrypt, input_file, output_file, key, iv)
}

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

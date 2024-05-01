mod crypter;
use crypter::Crypter;

mod decrypter;
pub use decrypter::Decrypter;

mod encrypter;
pub use encrypter::Encrypter;

mod usage;

#[cfg(test)]
pub(crate) mod helper {
    use std::fs::File;
    use std::io::{Read, Write};

    pub fn write_file(path: &str, reader: &mut impl Read) {
        let mut file = File::create(path).unwrap();
        let mut bytes = vec![];
        let _len = reader.read_to_end(&mut bytes).unwrap();
        file.write_all(&bytes).unwrap();
    }

    pub fn write_file_in_chunks(output_file: &str, reader: &mut impl Read, chunks: Vec<usize>) {
        let mut file = File::create(output_file).unwrap();
        for size in chunks {
            let mut buffer = vec![0; size];
            let loaded = reader.read(&mut buffer).unwrap();
            let _written = file.write(&buffer[..loaded]).unwrap();
        }
    }
}

use openssl::symm;
use std::io;
use std::io::{Read, Write};

pub struct Crypter<R> {
    inner: R,
    crypter: symm::Crypter,
    block_size: usize,
    finalized: bool,
    buffer: Vec<u8>,
    buffer_min_size: usize,
}

impl<R: Read> Crypter<R> {
    pub fn new(reader: R, crypter: symm::Crypter, block_size: usize) -> crate::Result<Self> {
        Ok(Crypter {
            inner: reader,
            crypter,
            block_size,
            finalized: false,
            buffer: vec![],
            buffer_min_size: 4096,
        })
    }

    fn finalize(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.finalized {
            return Ok(0);
        }
        let mut output = vec![0; self.block_size];
        let written = self.crypter.finalize(&mut output)?;
        self.finalized = true;

        let (moved, remaining) = move_buffer(buf, &output[..written])?;
        self.buffer = remaining.to_vec();
        Ok(moved)
    }
}

impl<R: Read> Read for Crypter<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.buffer.is_empty() {
            let (moved, remaining) = move_buffer(buf, &self.buffer)?;
            self.buffer = remaining.to_vec();
            return Ok(moved);
        }
        let mut buffer = vec![0; self.buffer_min_size.max(buf.len())];
        let loaded = self.inner.read(&mut buffer)?;

        let mut output = vec![0; loaded + self.block_size];
        let written = {
            let input = &buffer[..loaded];
            self.crypter.update(input, &mut output)?
        };
        if written == 0 {
            self.finalize(buf)
        } else {
            let (moved, remaining) = move_buffer(buf, &output[..written])?;
            self.buffer = remaining.to_vec();
            Ok(moved)
        }
    }
}

fn move_buffer<'a>(mut dst: &'a mut [u8], src: &'a [u8]) -> io::Result<(usize, &'a [u8])> {
    let moved = dst.len().min(src.len());
    let (bytes, remaining) = src.split_at(moved);
    dst.write_all(bytes)?;
    Ok((moved, remaining))
}

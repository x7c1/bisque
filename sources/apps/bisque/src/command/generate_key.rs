use crate::error::Error::KeyFileAlreadyExists;
use crate::{here, Result};
use bisque_cipher::generate_key;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    file_path: String,
}

pub fn run(args: Args) -> Result<()> {
    let file_path = args.file_path;
    let file_exists = fs::metadata(&file_path).is_ok();
    if file_exists {
        return Err(KeyFileAlreadyExists { path: file_path });
    }
    let key = generate_key();
    let mut file = File::create(file_path).map_err(here!())?;
    file.write_all(&key).map_err(here!())?;
    println!("key: {key:?}");

    Ok(())
}

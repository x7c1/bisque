use crate::Result;
use bisque_cipher::RandomBytes;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    file_path: String,
}

pub fn run(args: Args) -> Result<()> {
    let key = RandomBytes::generate();
    key.write_to_file(&args.file_path)?;
    Ok(())
}

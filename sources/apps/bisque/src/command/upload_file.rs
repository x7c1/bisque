use crate::Result;
use bisque_core::models::FilePath;
use bisque_core::AccessTokenLoader;
use bisque_core::{command::upload_file, BisqueClient};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    key_file: String,

    #[clap(long)]
    target_file: String,

    #[clap(long)]
    folder_id: String,

    #[clap(long)]
    session_file: String,
}

pub fn run(args: Args) -> Result<()> {
    let access_token_loader = AccessTokenLoader::setup(args.session_file)?;
    let access_token = access_token_loader.load()?;

    let drive_client = BisqueClient::new(access_token)?;
    drive_client.upload_file(upload_file::Params {
        key_file_path: FilePath::verify(args.key_file)?,
        src_file_path: FilePath::verify(args.target_file)?,
        dst_folder_id: args.folder_id,
    })?;
    Ok(())
}

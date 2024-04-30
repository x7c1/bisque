use bisque_core::AccessTokenLoader;
use bisque_core::{command::download_file, BisqueClient};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    key_file: String,

    #[clap(long)]
    file_name: String,

    #[clap(long)]
    folder_id: String,

    #[clap(long)]
    download_dir: String,

    #[clap(long)]
    session_file: String,
}

pub fn run(args: Args) -> crate::Result<()> {
    let access_token_loader = AccessTokenLoader::setup(args.session_file)?;
    let access_token = access_token_loader.load()?;

    let client = BisqueClient::new(access_token)?;
    client.download_file(download_file::Params {
        key_file_path: args.key_file,
        src_name: args.file_name.clone(),
        src_folder_id: args.folder_id,
        dst_file_path: PathBuf::from(args.download_dir).join(args.file_name),
    })?;
    Ok(())
}

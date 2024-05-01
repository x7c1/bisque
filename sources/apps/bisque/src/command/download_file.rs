use bisque_core::models::{DirPath, FilePath};
use bisque_core::AccessTokenLoader;
use bisque_core::{command::download_file, BisqueClient};
use bisque_google_drive::schemas::FileName;
use clap::Parser;

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
        key_file_path: FilePath::verify(args.key_file)?,
        src_name: FileName::new(args.file_name.clone())?,
        src_folder_id: args.folder_id,
        dst_dir_path: DirPath::verify(args.download_dir)?,
    })?;
    Ok(())
}

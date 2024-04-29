use crate::access_token_loader::AccessTokenLoader;
use crate::google_drive_client::{GoogleDriveClient, UploadFileParams};
use crate::Result;
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

    let drive_client = GoogleDriveClient::new(access_token)?;
    let file_path = args.target_file;
    drive_client.upload_file(UploadFileParams {
        key_file_path: args.key_file,
        src_file_path: file_path.to_string(),
        dst_name: file_path.split('/').last().unwrap_or("").to_string(),
        dst_folder_id: args.folder_id,
    })?;
    Ok(())
}

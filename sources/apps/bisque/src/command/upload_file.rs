use crate::access_token_loader::AccessTokenLoader;
use crate::google_drive_client::{GoogleDriveClient, UploadFileParams};
use crate::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    file_path: String,
    #[clap(long)]
    folder_id: String,
}

pub fn run(args: Args) -> Result<()> {
    let access_token_loader = AccessTokenLoader::setup()?;
    let access_token = access_token_loader.load()?;

    let drive_client = GoogleDriveClient::new(access_token);
    let file_path = args.file_path;
    drive_client.upload_file(UploadFileParams {
        src_file_path: file_path.to_string(),
        // TODO: remove unwrap
        dst_name: file_path.split('/').last().unwrap().to_string(),
        dst_folder_id: args.folder_id,
    })?;

    Ok(())
}

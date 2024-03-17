mod access_token_loader;
mod google_drive_client;
mod oauth_client;

mod command;
use crate::command::upload_file;

mod error;
pub use error::Result;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = upload_file::Args::parse();
    let result = upload_file::run(args);
    to_code(result)
}

fn to_code(result: Result<()>) -> ExitCode {
    match result {
        Ok(_) => {
            println!("[bisque] done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            println!("[bisque] failed: {cause:?}");
            ExitCode::FAILURE
        }
    }
}

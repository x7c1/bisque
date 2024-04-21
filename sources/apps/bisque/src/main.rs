mod access_token_loader;
mod command;
mod envs;
mod google_drive_client;
mod oauth_client;

mod error;
pub use error::Result;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let result = match args.subcommand {
        Subcommand::UploadFile(args) => command::upload_file::run(args),
        Subcommand::GenerateKey(args) => command::generate_key::run(args),
    };
    to_code(result)
}

fn to_code(result: Result<()>) -> ExitCode {
    match result {
        Ok(_) => {
            println!("[bisque] done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            println!("[bisque] failed: {cause:#?}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    GenerateKey(command::generate_key::Args),
    UploadFile(command::upload_file::Args),
}

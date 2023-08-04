use clap::{Args, Parser, Subcommand};

use crate::geometry::check_wkb;

mod geometry;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckWkb(CheckWkbArgs),
}

#[derive(Args)]
struct CheckWkbArgs {
    bytes: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CheckWkb(args) => {
            let bytes_string = &args.bytes;
            let bytes_array = serde_json::from_str::<Vec<u8>>(&bytes_string).expect("bytes array");
            check_wkb(bytes_array.as_slice());
            println!("Check WKB end");
        }
    }
}

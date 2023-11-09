use clap::{Parser, Subcommand};

use crate::b64;
use crate::url;
use crate::rot13;

#[derive(Parser)]
#[command(author = "Kevin K. <kbknapp@gmail.com>", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// b64 encode given param
    Enb64(b64::Enb64Args),

    /// b64 encode given param
    Deb64(b64::Deb64Args),

    /// URL encode given param
    Enurl(url::EnurlArgs),

    /// URL decode given param
    Deurl(url::DeurlArgs),

    /// Rot13 encode given param
    Enrot13(rot13::EnRot13Args),

    /// Rot13 encode given param
    Derot13(rot13::DeRot13Args),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Enb64(args) => {
            crate::b64::encode(args);
        }

        Commands::Deb64(args) => {
            b64::decode(args);
        }

        Commands::Enurl(args) => {
            url::encode(args);
        }

        Commands::Deurl(args) => {
            url::decode(args);
        }
        Commands::Enrot13(args) => {
            rot13::encode(args);
        }
        Commands::Derot13(args) => {
            rot13::decode(args);
        }
    }
}

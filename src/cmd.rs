use clap::{Parser, Subcommand};

use crate::b16;
use crate::b32;
use crate::b64;
use crate::hex;
use crate::html;
use crate::rot13;
use crate::url;

/// Simple encode/decode tool written in rust!
#[derive(Parser)]
#[command(
    author = "hatsat32 <suleymanergen32@gmail.com>",
    version,
    about,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// base64 encode given text
    Enb64(b64::Enb64Args),

    /// base64 encode given text
    Deb64(b64::Deb64Args),

    /// base32 encode given text
    Enb32(b32::Enb32Args),

    /// base32 encode given text
    Deb32(b32::Deb32Args),

    /// base16 encode given text
    Enb16(b16::EnB16Args),

    /// base16 encode given text
    Deb16(b16::DeB16Args),

    /// hex encode given text
    Enhex(hex::EnHexArgs),

    /// hex encode given text
    Dehex(hex::DeHexArgs),

    /// HTML encode given text
    Enhtml(html::EnHtmlArgs),

    /// HTML encode given text
    Dehtml(html::DeHtmlArgs),

    /// URL encode given text
    Enurl(url::EnurlArgs),

    /// URL decode given text
    Deurl(url::DeurlArgs),

    /// Rot13 encode given text
    Enrot13(rot13::EnRot13Args),

    /// Rot13 encode given text
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
            b64::encode(args);
        }
        Commands::Deb64(args) => {
            b64::decode(args);
        }

        Commands::Enb32(args) => {
            b32::encode(args);
        }
        Commands::Deb32(args) => {
            b32::decode(args);
        }

        Commands::Enb16(args) => {
            b16::encode(args);
        }
        Commands::Deb16(args) => {
            b16::decode(args);
        }

        Commands::Enhex(args) => {
            hex::encode(args);
        }
        Commands::Dehex(args) => {
            hex::decode(args);
        }

        Commands::Enhtml(args) => {
            html::encode(args);
        }
        Commands::Dehtml(args) => {
            html::decode(args);
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

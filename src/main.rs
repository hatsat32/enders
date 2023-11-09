use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// b64 encode given param
    Enb64 { toencode: String },

    /// b64 encode given param
    Deb64 { todecode: String },

    /// URL encode given param
    Enurl {
        toencode: String,

        /// Encode all characters
        #[arg(short, long)]
        all: bool,
    },

    /// URL decode given param
    Deurl { todecode: String },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Enb64 { toencode }) => {
            commands::b64::encode(toencode.to_string());
        }

        Some(Commands::Deb64 { todecode }) => {
            commands::b64::decode(todecode.to_string());
        }

        Some(Commands::Enurl { toencode, all }) => {
            commands::url::encode(toencode.to_string(), all.to_owned());
        }

        Some(Commands::Deurl { todecode }) => {
            commands::url::decode(todecode.to_string());
        }

        None => {
            println!("No command is provided. Please see usage!");
        }
    }
}

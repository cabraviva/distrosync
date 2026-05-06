use clap::{Parser, Subcommand};

mod commands;
mod aio;
mod utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Exports your whole system configuration")]
    Export,
    #[command(about = "Imports a previously exported configuration and applies it")]
    Import
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Export => {
            commands::export::run();
        }
        Commands::Import => {
            commands::import::run();
        }
    }
}
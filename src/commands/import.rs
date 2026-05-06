use std::env;

use colored::Colorize;

use crate::aio;

pub fn run() {
    println!("{}", "Starting system import.".cyan());

    let mut filepath = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("{} {}", "Error:".red().bold(), "There was an error during the import. Please create an issue on GitHub if it persists.".red());
            println!("{}", e);
            panic!()
        }
    };
    filepath = filepath.join("distrosync-export.zip");

    match aio::import::import_system(&filepath) {
        Ok(_) => {
            println!("{} Your configuration was imported from {}.", "Success:".green().bold(), filepath.display())
        },
        Err(err) => {
            println!("{} {}", "Error:".red().bold(), "There was an error during the import. Please create an issue on GitHub if it persists.".red());
            println!("{}", err);
        }
    };
}
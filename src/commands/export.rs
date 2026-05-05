use std::{env};

use colored::Colorize;
use crate::aio;


pub fn run() {
    println!("{}", "Starting full system export.".cyan());

    let mut filepath = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("{} {}", "Error:".red().bold(), "There was an error during the export. Please create an issue on GitHub if it persists.".red());
            println!("{}", e);
            panic!()
        }
    };
    filepath = filepath.join("distrosync-export.zip");

    match aio::export::export_system(&filepath) {
        Ok(_) => {
            println!("{} Your configuration was saved to {}.", "Success:".green().bold(), filepath.display())
        },
        Err(err) => {
            println!("{} {}", "Error:".red().bold(), "There was an error during the export. Please create an issue on GitHub if it persists.".red());
            println!("{}", err);
        }
    };
}
use std::{fs::File, io::Write, process::Command};

use colored::Colorize;
use zip::{ZipWriter, write::FileOptions};

pub fn export_to_zip(zip: &mut ZipWriter<File>) -> Result<(), String> {
    let out = Command::new("dconf").arg("dump").arg("/org/").output();

    match out {
        Ok(v) => {
            if !v.status.success() {
                return Err("Command 'dconf dump /org/' was not successful".into());
            }

            let out_str = String::from_utf8_lossy(&v.stdout);
            let options: FileOptions<()> = FileOptions::default();

            zip.start_file("dconf", options).map_err(|e| e.to_string())?;
            zip.write_all(out_str.as_bytes()).map_err(|e| e.to_string())?;
            zip.write_all(b"\n").map_err(|e| e.to_string())?;
        }
        Err(_) => {
            println!("{}", "Warning: Skipping dconf export because it does not seem to be installed".yellow());
            return Ok(());
        }
    }

    Ok(())
}
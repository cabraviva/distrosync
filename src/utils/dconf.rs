use std::{fs::File, io::{Read, Write}, process::{Command, Stdio}};

use colored::Colorize;
use zip::{ZipArchive, ZipWriter, write::FileOptions};

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

    println!("{}{}{} Exported dconf", "[".blue(), "✓".cyan(), "]".blue());
    Ok(())
}

pub fn import_from_zip(zip: &mut ZipArchive<File>) -> Result<(), String> {

    let file = zip.by_name("dconf");
    let mut fstr: String = "".into();

    match file {
        Ok(mut v) => {
            if !v.is_file() {
                return Err("dconf must be a file, but isn't".into())
            }

            
            v.read_to_string(&mut fstr).map_err(|e| e.to_string())?;
        },
        Err(_) => {
            println!("{}", "Warning: Skipping dconf export because it does not seem to be installed".yellow());
            return Ok(());
        }
    }

    let child = Command::new("dconf").arg("load").arg("/org/").stdin(Stdio::piped()).spawn();

    match child {
        Ok(mut v) => {
            if let Some(mut stdin) = v.stdin.take() {
                stdin.write_all(fstr.as_bytes()).unwrap();
            }
        },
        Err(_) => {
            println!("{}", "Warning: Skipping dconf import because it does not seem to be installed".yellow());
            return Ok(());
        }
    }

    println!("{}{}{} Imported dconf", "[".blue(), "✓".cyan(), "]".blue());

    Ok(())
}
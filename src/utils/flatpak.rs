use std::{fs::File, io::Write, process::Command};
use colored::Colorize;
use zip::{ZipWriter, write::FileOptions};

pub fn flatpak_installed() -> bool {
    let out = Command::new("flatpak").arg("--version").output();

    match out {
        Ok(v) => {
            return v.status.success(); // true if successful
        }
        Err(e) => {
            println!("Command 'flatpak --version' failed: {}", e);
            return false;
        }
    }
}

pub fn export_to_zip(zip: &mut ZipWriter<File>) -> Result<(), String> {
    const FLATPAK_REMOTES_EXPORT_SCRIPT: &str = r#"
flatpak remotes --system --columns=name,url \
| awk '{printf "flatpak remote-add --if-not-exists --system %s %s\n", $1, $2}'

flatpak remotes --user --columns=name,url \
| awk '{printf "flatpak remote-add --if-not-exists --user %s %s\n", $1, $2}'
"#;

    if !flatpak_installed() {
        println!("{}", "Warning: Skipping flatpak export because it does not seem to be installed".yellow());
        return Ok(());
    }

    // 1. Export remotes
    let out_remotes = Command::new("sh").arg("-c").arg(FLATPAK_REMOTES_EXPORT_SCRIPT).output().expect("sh is not executable on your system");
    if !out_remotes.status.success()  {
        return Err("Flatpak remotes export script was not successful".into());
    }
    let remote_str = String::from_utf8_lossy(&out_remotes.stdout);
    
    // Add to zip
    let options: FileOptions<()> = FileOptions::default();
    zip.add_directory("flatpak", options).map_err(|e| e.to_string())?;
    zip.start_file("flatpak/remotes", options).map_err(|e| e.to_string())?;

    for line in remote_str.lines() {
        let trimmed_cmd = line.trim();
        if trimmed_cmd.is_empty()  { continue };

        zip.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        zip.write_all(b"\n").map_err(|e| e.to_string())?;
    }


    // 2. Export packages

    Ok(())
}
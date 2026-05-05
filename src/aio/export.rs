use std::{fs::File, io::Write, path::PathBuf};

use zip::{ZipWriter, write::FileOptions};

pub fn export_system(filepath: &PathBuf) -> Result<(), String> {
    let zipfile = File::create(&filepath).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(zipfile);

    let options: FileOptions<()> = FileOptions::default();

    zip.start_file("readme.txt", options).map_err(|e| e.to_string())?;

    zip.write_all(b"Hello, World!\n").map_err(|e| e.to_string())?;



    zip.finish().map_err(|e| e.to_string())?;

    Ok(())
}
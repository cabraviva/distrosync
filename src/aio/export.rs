use std::{fs::File, path::PathBuf};

use zip::ZipWriter;

use crate::utils::{flatpak, dconf};

pub fn export_system(filepath: &PathBuf) -> Result<(), String> {
    let zipfile = File::create(&filepath).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(zipfile);

    flatpak::export_to_zip(&mut zip)?;
    dconf::export_to_zip(&mut zip)?;

    zip.finish().map_err(|e| e.to_string())?;

    Ok(())
}

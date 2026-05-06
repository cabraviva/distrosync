use std::{fs::File, path::PathBuf};

use zip::ZipArchive;

use crate::utils::{dconf, flatpak};

pub fn import_system(filepath: &PathBuf) -> Result<(), String> {
    let zipfile = File::open(&filepath).map_err(|e| e.to_string())?;
    let mut zip = ZipArchive::new(zipfile).map_err(|e| e.to_string())?;

    flatpak::import_from_zip(&mut zip)?;
    dconf::import_from_zip(&mut zip)?;

    Ok(())
}

use std::fs::File;

use zip::ZipWriter;

// TODO: (via: dconf dump /org/)
pub fn export_to_zip(zip: &mut ZipWriter<File>) -> Result<(), String> {
    Ok(())
}
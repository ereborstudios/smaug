use log::*;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub(crate) fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry.expect("Could not find file");
        let entry = entry.path();

        let new_path = entry
            .to_str()
            .unwrap()
            .replace(source.to_str().unwrap(), destination.to_str().unwrap());
        let new_path = Path::new(&new_path);

        if entry.is_file() && !entry.to_str().unwrap().contains("/.git/") {
            trace!(
                "Creating directory {}",
                new_path.parent().and_then(|p| p.to_str()).unwrap()
            );
            fs::create_dir_all(new_path.parent().unwrap())?;
            trace!(
                "Copying file from {} to {}",
                entry.to_str().unwrap(),
                new_path.to_str().unwrap()
            );
            fs::copy(entry, new_path)?;
        }
    }

    Ok(())
}

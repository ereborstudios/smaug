use log::*;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_directory<P: AsRef<Path>>(source: &P, destination: &P) -> io::Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry.expect("Could not find directory");
        let entry = entry.path();
        let relative = entry.strip_prefix(source.as_ref()).unwrap();
        let new_path = destination.as_ref().join(relative);

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

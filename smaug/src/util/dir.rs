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

        if entry.is_file() && !is_git_dir(entry.to_str().unwrap()) {
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

fn is_git_dir(path: &str) -> bool {
    path.contains("/.git/") || path.contains("\\.git\\")
}

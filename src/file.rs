use log::*;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;
use zip_extensions::zip_extract;

pub fn unzip(source: &Path, destination: &Path) {
    if destination.is_dir() {
        trace!("Removing directory {}", destination.to_str().unwrap());
        fs::remove_dir_all(destination).unwrap();
    }
    trace!("Extracting zip to {}", destination.to_str().unwrap());
    zip_extract(&source.to_path_buf(), &destination.to_path_buf()).unwrap();

    let files = fs::read_dir(destination)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let file_count = files.iter().count();
    debug!("Found {} files after unzipping", file_count);
    let maybe_dir = files.first().unwrap();

    if file_count == 1 && maybe_dir.is_dir() {
        debug!("Only file is a directory. Moving files in directory to parent.");
        for entry in WalkDir::new(maybe_dir) {
            let entry = entry.unwrap();
            let entry = entry.path();

            let new_path = entry
                .to_str()
                .unwrap()
                .replace(maybe_dir.to_str().unwrap(), destination.to_str().unwrap());
            let new_path = Path::new(&new_path);

            if entry.is_file() {
                trace!(
                    "Creating directory {}",
                    new_path.parent().and_then(|p| p.to_str()).unwrap()
                );
                fs::create_dir_all(new_path.parent().unwrap()).unwrap();
                trace!(
                    "Copying file from {} to {}",
                    entry.to_str().unwrap(),
                    new_path.to_str().unwrap()
                );
                fs::copy(entry, new_path).unwrap();
            }
        }

        trace!("Removing directory {}", maybe_dir.to_str().unwrap());
        fs::remove_dir_all(maybe_dir).unwrap();
    }
}

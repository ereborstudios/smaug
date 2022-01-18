use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::*;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_directory<P: AsRef<Path>>(source: &P, destination: P) -> io::Result<()> {
    let mut ignore_builder = GitignoreBuilder::new(source);
    let ignore_file = source.as_ref().join(".smaugignore");

    if ignore_file.is_file() {
        ignore_builder.add(ignore_file);
    }

    let ignore = ignore_builder
        .build()
        .expect("Could not parse smaugignore file");

    for entry in WalkDir::new(source) {
        let entry = entry.expect("Could not find directory");
        let entry = entry.path();
        let relative = entry.strip_prefix(source.as_ref()).unwrap();
        let new_path = destination.as_ref().join(relative);

        if entry.is_file() && !is_ignored(entry, &ignore) {
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

fn is_ignored(path: &Path, ignore: &Gitignore) -> bool {
    if matches_ignore(path, ignore) {
        trace!("Ignoring {:?}", path);
        return true;
    }

    false
}

fn matches_ignore(path: &Path, ignore: &Gitignore) -> bool {
    ignore
        .matched_path_or_any_parents(path, path.is_dir())
        .is_ignore()
        || is_git_dir(path.to_string_lossy().as_ref())
}

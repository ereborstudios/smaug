use crate::dependency::Dependency;
use crate::source::Source;
use crate::sources::dir_source::DirSource;
use log::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use zip_extensions::zip_extract;

#[derive(Debug)]
pub struct FileSource {
    pub path: PathBuf,
}

impl Source for FileSource {
    fn install(&self, dependency: &Dependency, destination: &PathBuf) -> std::io::Result<()> {
        trace!("Installing file at {}", self.path.display());
        let cached = crate::smaug::cache_dir().join(dependency.clone().name);

        if cached.is_dir() {
            std::fs::remove_dir_all(cached.clone())?;
        }

        trace!("Extracting zip to {}", cached.display());
        zip_extract(&self.path.to_path_buf(), &cached)?;

        trace!(
            "Finding top level package directory in {}",
            cached.display()
        );

        match find_package_dir(&cached) {
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No Smaug.toml file found in {}", cached.display()).as_str(),
            )),
            Some(dir) => DirSource { path: dir }.install(dependency, &destination),
        }
    }
}

fn find_package_dir(path: &PathBuf) -> Option<PathBuf> {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        if entry.file_name() == "Smaug.toml" {
            return Some(entry.path().parent().unwrap().to_path_buf());
        }
    }

    None
}

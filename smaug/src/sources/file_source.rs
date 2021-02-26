use crate::source::Source;
use crate::sources::dir_source::DirSource;
use crate::{dependency::Dependency, resolver::Resolver};
use log::*;
use std::path::PathBuf;
use walkdir::WalkDir;
use zip_extensions::zip_extract;

#[derive(Clone, Debug)]
pub struct FileSource {
    pub path: PathBuf,
}

impl Source for FileSource {
    fn install(
        &self,
        resolver: &mut Resolver,
        dependency: &Dependency,
        destination: &PathBuf,
    ) -> std::io::Result<()> {
        trace!("Installing file at {}", self.path.display());
        let cached = crate::smaug::cache_dir().join(dependency.clone().name);

        rm_rf::ensure_removed(cached.clone()).expect("Couldn't remove directory");

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
            Some(dir) => DirSource { path: dir }.install(resolver, dependency, &destination),
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

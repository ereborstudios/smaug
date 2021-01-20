use crate::source::Source;
use crate::sources::file_source::FileSource;
use crate::{dependency::Dependency, registry::Registry};
use log::*;
use std::fs::File;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct UrlSource {
    pub url: String,
}

impl Source for UrlSource {
    fn install(
        &self,
        registry: &mut Registry,
        dependency: &Dependency,
        destination: &PathBuf,
    ) -> std::io::Result<()> {
        trace!("Downloading Url from {}", self.url);
        let file_name = format!("{}.zip", dependency.clone().name);
        let cached = crate::smaug::cache_dir().join(file_name);

        if cached.exists() {
            std::fs::remove_file(cached.clone())?;
        }

        trace!("Downloading package to {}", cached.display());
        std::fs::create_dir_all(cached.parent().unwrap())?;
        let mut file = File::create(cached.clone())?;
        let response = reqwest::blocking::get(self.url.as_str());

        match response {
            Err(..) => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Couldn't download file.",
            )),
            Ok(mut response) => {
                std::io::copy(&mut response, &mut file)?;
                FileSource { path: cached }.install(registry, dependency, destination)
            }
        }
    }
}

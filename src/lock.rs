use crate::dependency::Dependency;
use crate::project_config::ProjectConfig;
use log::*;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct FileLock {
    pub(crate) package: String,
    pub(crate) source: PathBuf,
    pub(crate) destination: PathBuf,
    pub(crate) digest: String,
    pub(crate) require: bool,
}

#[derive(Debug)]
pub(crate) struct Lock {
    pub(crate) files: Vec<FileLock>,
}

impl Lock {
    pub fn from_config(config: &ProjectConfig) -> Lock {
        trace!("Calculating lock");
        let mut files: Vec<FileLock> = vec![];

        for dependency in config.dependencies.iter() {
            Dependency::from_config(&dependency)
                .ok()
                .map(|dep| dep.cache())
                .map(|source| parse_package(&source, dependency.to_owned().name.unwrap()))
                .map(|mut f| files.append(&mut f))
                .unwrap()
        }

        Lock { files }
    }
}

fn parse_package(path: &PathBuf, name: String) -> Vec<FileLock> {
    let config = ProjectConfig::load(path.join("Smaug.toml"));
    debug!("PackageConfig: {:?}", config);
    let mut files: Vec<FileLock> = vec![];

    for file in config.files {
        trace!("Parsing file: {:?}", file);
        let source = path.join(file.from.clone());
        let file_lock = FileLock {
            source,
            package: name.clone(),
            destination: PathBuf::from(file.to),
            require: file.require,
            digest: String::from("asdfasdf"),
        };

        files.push(file_lock);
    }

    files
}

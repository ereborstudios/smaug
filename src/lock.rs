use crate::dependency::Dependency;
use crate::digest;
use crate::project_config::ProjectConfig;
use log::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct FileLock {
    pub(crate) package: String,
    pub(crate) source: PathBuf,
    pub(crate) destination: PathBuf,
    pub(crate) digest: String,
    pub(crate) require: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Lock {
    pub(crate) files: Vec<FileLock>,
}

#[derive(Debug)]
pub enum LockError {
    ConflictingPackages {
        packages: (String, String),
        file: String,
    },
}

type LockResult = Result<Lock, LockError>;

impl Lock {
    pub fn from_config(config: &ProjectConfig) -> LockResult {
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

        let lock = Lock { files };
        match validate(&lock) {
            Ok(()) => Ok(lock),
            Err(error) => Err(error),
        }
    }
}

fn validate(lock: &Lock) -> Result<(), LockError> {
    validate_no_conflicting_files(lock)
}

fn validate_no_conflicting_files(lock: &Lock) -> Result<(), LockError> {
    let mut map: HashMap<&str, &FileLock> = HashMap::new();
    let files = lock.files.iter();

    for file in files {
        let destination = file.destination.to_str().unwrap();
        let package = file.package.clone();

        if map.contains_key(destination) {
            let conflicting = map.get(&destination).unwrap();
            let conflicting_package = conflicting.package.clone();

            let original_digest = digest::file(file.source.as_path()).unwrap();
            let conflicting_digest = digest::file(conflicting.source.as_path()).unwrap();

            if original_digest != conflicting_digest {
                let error = LockError::ConflictingPackages {
                    packages: (package, conflicting_package),
                    file: String::from(destination),
                };

                return Err(error);
            }
        }

        map.insert(destination, &file);
    }

    Ok(())
}

fn parse_package(path: &PathBuf, name: String) -> Vec<FileLock> {
    let config = ProjectConfig::load(path.join("Smaug.toml"));
    debug!("PackageConfig: {:?}", config);
    let mut files: Vec<FileLock> = vec![];

    for file in config.files {
        trace!("Parsing file: {:?}", file);
        let source = path.join(file.from.clone());
        let digest = digest::file(&source).unwrap();

        let file_lock = FileLock {
            source,
            digest,
            package: name.clone(),
            destination: PathBuf::from(file.to),
            require: file.require,
        };
        debug!("FileLock: {:?}", file_lock);

        files.push(file_lock);
    }

    files
}

use crate::dependency::Dependency;
use crate::project_config::ProjectConfig;
use log::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{collections::HashMap, process};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Lock {
    pub packages: Vec<PackageLock>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub removed_packages: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PackageLock {
    pub name: String,
    pub version: String,
    pub requires: Vec<String>,
    #[serde(skip_serializing)]
    pub cache: Option<PathBuf>,
}

#[derive(Debug)]
pub(crate) enum LockError {
    ParsingFailed(String),
}

type LockResult = Result<Lock, LockError>;

impl Lock {
    pub fn load<P: AsRef<Path>>(path: &P) -> LockResult {
        let file = path.as_ref();

        if file.exists() {
            let contents = fs::read_to_string(file).unwrap();
            match toml::from_str(&contents) {
                Err(message) => {
                    println!("{:?}", message);
                    return Err(LockError::ParsingFailed(message.to_string()));
                }
                Ok(lock) => return Ok(lock),
            }
        }

        Ok(Lock::default())
    }

    pub fn update_from_config(&self, config: &ProjectConfig) -> LockResult {
        let mut installed_packages = HashMap::new();
        let mut config_packages = HashSet::new();
        let mut updated_packages = HashSet::new();
        let mut package_map = HashMap::new();
        let mut packages = vec![];

        for package in self.packages.iter() {
            installed_packages.insert(package.name.clone(), package);
        }

        let dependencies = config
            .dependencies
            .iter()
            .map(|dep| Dependency::from_config(dep).unwrap());

        for package in dependencies {
            package_map.insert(package.name.clone(), package.clone());
            config_packages.insert(package.name.clone());
            if !installed_packages.contains_key(&package.name) {
                updated_packages.insert(package.name.clone());
            } else {
                let package = installed_packages.get(&package.name).unwrap();

                let package_lock = PackageLock {
                    name: package.name.clone(),
                    version: package.version.clone(),
                    requires: package.requires.clone(),
                    cache: package.cache.clone(),
                };
                packages.push(package_lock);
            }
        }

        for package_name in updated_packages {
            let dependency_config = package_map.get(&package_name).unwrap();
            let cache = dependency_config.cache();
            let package_config = ProjectConfig::load(&cache.path.join("Smaug.toml")).unwrap();
            debug!("Package Config: {:?}", package_config);

            if package_config.package.is_none() {
                let message = format!(
                    "Malformed package: {}",
                    package_config.project.name.unwrap()
                );

                crate::smaug::print_error(message);
                process::exit(exitcode::DATAERR);
            }

            let package_lock = PackageLock {
                name: package_name,
                version: cache.version.clone(),
                requires: package_config.package.unwrap().requires,
                cache: Some(cache.path.clone()),
            };
            packages.push(package_lock);
        }

        let installed_package_names: HashSet<String> = installed_packages.keys().cloned().collect();
        let removed_packages = installed_package_names
            .difference(&config_packages)
            .map(String::from);

        let lock = Lock {
            packages,
            removed_packages: removed_packages.collect(),
        };

        Ok(lock)
    }
}

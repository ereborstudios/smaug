use crate::file;
use crate::git;
use crate::project_config::Dependency as DependencyConfig;
use crate::smaug;
use crate::url;
use ::url::Url;
use log::*;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub source: DependencySource,
}

#[derive(Debug, Clone)]
pub enum DependencySource {
    Dir {
        path: PathBuf,
    },
    File {
        path: PathBuf,
    },
    Git {
        repo: String,
        branch: Option<String>,
    },
    Url {
        location: Url,
    },
}

impl Dependency {
    pub fn from_config(config: &DependencyConfig) -> Result<Dependency, String> {
        let name = config.name.as_ref().unwrap().clone();

        if config.repo.is_some() {
            let source = DependencySource::Git {
                repo: config.repo.as_ref().unwrap().clone(),
                branch: config.branch.clone(),
            };

            let dependency = Dependency { name, source };

            Ok(dependency)
        } else if config.dir.is_some() {
            let source = DependencySource::Dir {
                path: Path::new(&config.dir.as_ref().unwrap()).to_path_buf(),
            };

            let dependency = Dependency { name, source };

            Ok(dependency)
        } else if config.url.is_some() {
            let source = DependencySource::Url {
                location: config.url.as_ref().unwrap().clone(),
            };

            let dependency = Dependency { name, source };

            Ok(dependency)
        } else if config.file.is_some() {
            let source = DependencySource::File {
                path: Path::new(&config.file.as_ref().unwrap()).to_path_buf(),
            };

            let dependency = Dependency { name, source };

            Ok(dependency)
        } else {
            Err(String::from("Could not parse dependency"))
        }
    }

    pub fn cache(&self) -> PathBuf {
        let cache_dir = smaug::cache_dir();
        debug!("Cache directory {}", cache_dir.to_str().unwrap());

        match self.source.clone() {
            DependencySource::Git { repo, branch } => {
                let clone = git::Clone {
                    repo: repo.to_owned(),
                    branch: branch.to_owned(),
                };
                let destination = cache_dir.join(self.name.clone());
                clone.clone(&destination);

                destination
            }
            DependencySource::Dir { path: dir } => dir.to_owned(),
            DependencySource::Url { location } => {
                let source = cache_dir.join(format!("{}.zip", self.name.clone()));
                let destination = cache_dir.join(self.name.clone());
                url::download(&location, &source);
                file::unzip(&source, &destination);

                destination
            }
            DependencySource::File { path: zip } => {
                let destination = cache_dir.join(self.name.clone());
                file::unzip(&zip, &destination);

                destination
            }
        }
    }
}

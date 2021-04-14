use url_source::UrlSource;

use crate::resolver::Install;
use crate::sources::file_source::FileSource;
use crate::sources::registry_source::RegistrySource;
use crate::{config::DependencyOptions, sources::git_source::GitSource};
use crate::{dependency::Dependency, sources::url_source};
use crate::{resolver::Resolver, sources::dir_source::DirSource};
use log::*;
use std::path::Path;

pub trait Source: SourceClone {
    fn install(&self, dependency: &Dependency, path: &Path) -> std::io::Result<()>;

    fn installed(&self, dependency: &Dependency, destination: &Path) -> bool {
        let destination = destination.join(dependency.clone().name);
        destination.exists()
    }

    fn update_resolver(
        &self,
        resolver: &mut Resolver,
        dependency: &Dependency,
        destination: &Path,
    ) {
        let project_dir = destination.parent().unwrap();
        let destination = destination.join(dependency.clone().name);
        let config_path = destination.join("Smaug.toml");
        let config = crate::config::load(&config_path).expect("Could not find Smaug.toml");
        debug!("Package config: {:?}", config);
        let package = config.package.expect("No package configuration found.");

        for (from, to) in package.installs {
            let install_source = from.to_path(destination.as_path());
            let install_destination = to.to_path(project_dir);

            let install = Install {
                from: install_source,
                to: install_destination,
            };

            resolver.installs.push(install);
        }

        let mut requires = package
            .requires
            .iter()
            .map(|require| {
                let package_file = require.to_path(destination.clone());
                trace!("Checking package file {:?}", package_file);

                if package_file.exists() {
                    trace!("package file exists");
                    format!("smaug/{}/{}", dependency.name, require)
                } else {
                    trace!("package file does not exists");
                    require.to_string()
                }
            })
            .collect();
        resolver.requires.append(&mut requires);
    }
}

pub trait SourceClone {
    fn clone_box(&self) -> Box<dyn Source>;
}

pub fn from_dependency_options(options: &DependencyOptions) -> Option<Box<dyn Source>> {
    match options {
        DependencyOptions::Git {
            repo,
            branch,
            rev,
            tag,
        } => Some(Box::new(GitSource {
            repo: repo.clone(),
            branch: branch.clone(),
            rev: rev.clone(),
            tag: tag.clone(),
        })),
        DependencyOptions::Dir { dir: path } => Some(Box::new(DirSource {
            path: path.to_path_buf(),
        })),
        DependencyOptions::File { file: path } => Some(Box::new(FileSource {
            path: path.to_path_buf(),
        })),
        DependencyOptions::Url { url } => Some(Box::new(UrlSource {
            url: url.to_string(),
        })),
        DependencyOptions::Registry { version } => Some(Box::new(RegistrySource {
            version: version.to_string(),
        })),
    }
}

impl<T> SourceClone for T
where
    T: 'static + Source + Clone,
{
    fn clone_box(&self) -> Box<dyn Source> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Source> {
    fn clone(&self) -> Box<dyn Source> {
        self.clone_box()
    }
}

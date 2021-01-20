use crate::dependency;
use crate::{config, source::Source};
use config::{Config, DependencyOptions};
use dependency::Dependency;
use log::*;
use semver::VersionReq;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Default)]
pub struct Registry {
    pub requirements: Vec<Dependency>,
    pub source_map: HashMap<String, Box<dyn Source>>,
    pub installs: Vec<Install>,
    pub requires: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Install {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Registry {
    pub fn install(&mut self, destination: PathBuf) -> std::io::Result<()> {
        let reqs = self.requirements.clone();
        let sources = self.source_map.clone();

        for dependency in reqs.iter() {
            let source = sources.get(&dependency.name).unwrap();

            source.install(self, dependency, &destination)?;
        }

        Ok(())
    }

    pub fn add_source(&mut self, name: String, source: Box<dyn Source>) {
        self.source_map.entry(name).or_insert(source);
    }

    pub fn add_requirement(&mut self, dependency: Dependency) {
        self.requirements.push(dependency);
    }
}

pub fn new_from_config(config: &Config) -> Registry {
    let mut registry = Registry::default();

    for (name, dependency_options) in config.dependencies.iter() {
        let name = String::from(name);
        let version = match dependency_options {
            DependencyOptions::Registry { version, .. } => version.clone(),
            _ => VersionReq::any(),
        };

        debug!("{:?}", dependency_options);
        let source = crate::source::from_dependency_options(dependency_options)
            .expect("could not create source");
        let dependency = Dependency {
            name: name.clone(),
            version,
        };

        registry.add_requirement(dependency.clone());
        registry.add_source(dependency.name, source);
    }

    registry
}

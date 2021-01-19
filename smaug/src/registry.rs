use crate::dependency;
use crate::{config, source::Source};
use config::{Config, DependencyOptions};
use dependency::Dependency;
use log::*;
use semver::VersionReq;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct Registry {
    pub requirement_map: HashMap<String, Vec<Dependency>>,
    pub source_map: HashMap<String, Box<dyn Source>>,
}

impl Registry {
    pub fn install(&self, destination: PathBuf) -> std::io::Result<()> {
        for (name, requirements) in self.requirement_map.iter() {
            let requirement = requirements.first().unwrap();
            let source = self.source_map.get(name).unwrap();

            source.install(requirement, &destination)?;
        }

        Ok(())
    }

    pub fn add_source(&mut self, name: String, source: Box<dyn Source>) {
        self.source_map.entry(name).or_insert(source);
    }

    pub fn add_requirement(&mut self, dependency: Dependency) {
        if self.requirement_map.contains_key(&dependency.name) {
            self.requirement_map
                .get_mut(&dependency.name)
                .unwrap()
                .push(dependency);
        } else {
            self.requirement_map
                .insert(dependency.name.clone(), vec![dependency]);
        }
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

use crate::dependency::Dependency;
use crate::resolver::Install;
use crate::resolver::Resolver;
use crate::source::Source;
use log::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DirSource {
    pub path: PathBuf,
}

impl Source for DirSource {
    fn install(
        &self,
        resolver: &mut Resolver,
        dependency: &Dependency,
        destination: &PathBuf,
    ) -> std::io::Result<()> {
        let project_dir = destination.parent().unwrap();
        let destination = destination.join(dependency.clone().name);
        trace!(
            "Installing directory from {} to {}",
            self.path.display(),
            destination.display()
        );

        crate::util::dir::copy_directory(&self.path, &destination)?;

        let config_path = destination.join("Smaug.toml");
        let config = crate::config::load(&config_path).expect("Could not find Smaug.toml");
        debug!("Package config: {:?}", config);
        let package = config.package.expect("No package configuration found.");

        for (from, to) in package.installs {
            let install_source = destination.join(from);
            let install_destination = project_dir.join(to);

            let install = Install {
                from: install_source,
                to: install_destination,
            };

            resolver.installs.push(install);
        }

        let mut requires = package.requires;
        resolver.requires.append(&mut requires);

        Ok(())
    }
}

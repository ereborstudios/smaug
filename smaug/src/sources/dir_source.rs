use crate::dependency::Dependency;
use crate::source::Source;
use log::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DirSource {
    pub path: PathBuf,
}

impl Source for DirSource {
    fn install(&self, dependency: &Dependency, destination: &PathBuf) -> std::io::Result<()> {
        let destination = destination.join(dependency.clone().name);
        trace!(
            "Installing directory from {} to {}",
            self.path.display(),
            destination.display()
        );

        crate::util::dir::copy_directory(&self.path, &destination)?;

        Ok(())
    }
}

use crate::dependency::Dependency;
use crate::source::Source;
use log::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DirSource {
    pub path: PathBuf,
}

impl Source for DirSource {
    fn install(&self, dependency: &Dependency, destination: &PathBuf) -> std::io::Result<()> {
        let project_dir = destination.parent().unwrap();
        let source = project_dir.join(self.path.to_path_buf());
        let destination = destination.join(dependency.clone().name);
        trace!(
            "Installing directory from {} to {}",
            source.display(),
            destination.display()
        );

        crate::util::dir::copy_directory(&source, &destination)?;

        Ok(())
    }
}

use crate::dependency::Dependency;
use crate::source::Source;
use log::*;
use question::{Answer, Question};
use std::path::PathBuf;

#[derive(Debug)]
pub struct DirSource {
    pub path: PathBuf,
}

impl Source for DirSource {
    fn install(&self, dependency: &Dependency, destination: &PathBuf) -> std::io::Result<()> {
        let project_dir = destination.parent().unwrap();
        let destination = destination.join(dependency.clone().name);
        trace!(
            "Installing directory from {} to {}",
            self.path.display(),
            destination.display()
        );

        crate::util::dir::copy_directory(&self.path, &destination)?;
        install_files(&destination, &project_dir.to_path_buf())?;

        Ok(())
    }
}

fn install_files(source: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
    let config_file = source.join("Smaug.toml");
    let config = crate::config::load(&config_file).expect("Could not find Smaug.toml");
    debug!("Package Config: {:?}", config);

    for (from, to) in config
        .package
        .expect("Missing package declaration")
        .installs
    {
        let file_source = source.join(from);
        let file_destination = destination.join(to);

        if can_install_file(&file_source, &file_destination) {
            trace!(
                "Copying file from {} to {}",
                file_source.display(),
                file_destination.display()
            );
            std::fs::create_dir_all(file_destination.parent().unwrap())?;
            std::fs::copy(file_source, file_destination)?;
        }
    }

    Ok(())
}

fn can_install_file(source: &PathBuf, destination: &PathBuf) -> bool {
    if !destination.exists() {
        return true;
    }

    let source_digest = crate::util::digest::file(source).unwrap();
    let destination_digest = crate::util::digest::file(destination).unwrap();
    debug!(
        "Source: {}, Destination: {}",
        source_digest, destination_digest
    );

    let changed = source_digest != destination_digest;

    if !changed {
        return true;
    }

    let question = format!(
        "{} has changed since the last install. Do you want to overwrite it?",
        destination.display()
    );

    let answer = Question::new(question.as_str())
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    answer == Answer::YES
}

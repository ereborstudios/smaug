use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use log::*;
use question::{Answer, Question};
use resolver::Resolver;
use serde::Serialize;
use smaug::resolver;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

#[derive(Debug)]
pub struct Install;

impl Command for Install {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Install Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let canonical = std::fs::canonicalize(directory)?;
        let path = Path::new(&canonical);
        let path = std::fs::canonicalize(&path).expect("Could not find path");

        let config_path = path.join("Smaug.toml");

        let config = smaug::config::load(&config_path)?;
        debug!("Smaug config: {:?}", config);

        let mut registry = resolver::new_from_config(&config);

        match registry.install(path.join("smaug")) {
            Ok(()) => {
                debug!("{:?}", registry.requires);
                install_files(&registry)?;
                write_index(&registry, &path)?;
                Ok(Box::new("Successfully installed your dependencies."))
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}

#[derive(Debug, Serialize)]
struct Index {
    requires: Vec<String>,
}

static INDEX_TEMPLATE: &str = include_str!("../../templates/smaug.rb.template");
fn write_index(resolver: &Resolver, path: &Path) -> std::io::Result<()> {
    trace!("Writing index");
    let mut tt = TinyTemplate::new();

    tt.add_template("smaug.rb", INDEX_TEMPLATE)
        .expect("couldn't add template.");

    let context = Index {
        requires: resolver.requires.clone(),
    };

    debug!("Context: {:?}", context);

    let rendered = tt
        .render("smaug.rb", &context)
        .expect("Could not render smaug.rb");

    let index_path = path.join("smaug.rb");
    trace!("Writing index to {}", index_path.display());
    std::fs::write(index_path, rendered).expect("Could not write file");

    info!("Add `require \"smaug.rb\" to the top of your main.rb");

    Ok(())
}

fn install_files(resolver: &Resolver) -> std::io::Result<()> {
    trace!("Installing files");
    debug!("{:?}", resolver.installs);
    for install in resolver.installs.iter() {
        if can_install_file(&install.from, &install.to) {
            trace!(
                "Copying file from {} to {}",
                install.from.display(),
                install.to.display()
            );
            std::fs::create_dir_all(install.to.parent().unwrap())?;
            std::fs::copy(install.from.clone(), install.to.clone())?;
        }
    }

    Ok(())
}

fn can_install_file(source: &PathBuf, destination: &PathBuf) -> bool {
    if !destination.exists() {
        return true;
    }

    let source_digest = smaug::util::digest::file(source).unwrap();
    let destination_digest = smaug::util::digest::file(destination).unwrap();
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

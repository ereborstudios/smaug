use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use question::{Answer, Question};
use resolver::Resolver;
use serde::Serialize;
use smaug_lib::{dependency::Dependency, resolver};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;
use dunce;

#[derive(Debug)]
pub struct Install;

#[derive(Debug, Display, Serialize)]
#[display(fmt = "Successfully installed your dependencies.")]
pub struct InstallResult {
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Display, Error, Serialize)]
enum Error {
    #[display(fmt = "Failed to install your dependencies.")]
    InstallFailed,
    #[display(fmt = "Could not find Smaug.toml at {}", "path.display()")]
    FileNotFound { path: PathBuf },
    #[display(fmt = "Couldn't load Smaug configuration.")]
    Config { path: PathBuf },
}

impl Command for Install {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Install Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let canonical = match dunce::canonicalize(directory) {
            Ok(dir) => dir,
            Err(..) => {
                return Err(Box::new(Error::FileNotFound {
                    path: Path::new(directory).to_path_buf(),
                }))
            }
        };
        let path = Path::new(&canonical);
        let path = dunce::canonicalize(&path).expect("Could not find path");

        let config_path = path.join("Smaug.toml");

        let config = match smaug_lib::config::load(&config_path) {
            Ok(config) => config,
            Err(..) => return Err(Box::new(Error::Config { path: config_path })),
        };
        debug!("Smaug config: {:?}", config);

        let mut registry = resolver::new_from_config(&config);

        match registry.install(path.join("smaug")) {
            Ok(dependencies) => {
                debug!("{:?}", registry.requires);
                if install_files(&registry).is_err() {
                    return Err(Box::new(Error::InstallFailed));
                }

                write_index(&registry, &path);

                Ok(Box::new(InstallResult { dependencies }))
            }
            Err(..) => Err(Box::new(Error::InstallFailed)),
        }
    }
}

#[derive(Debug, Serialize)]
struct Index {
    requires: Vec<String>,
}

static INDEX_TEMPLATE: &str = include_str!("../../templates/smaug.rb.template");
fn write_index(resolver: &Resolver, path: &Path) {
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
}

fn install_files(resolver: &Resolver) -> std::io::Result<()> {
    trace!("Installing files");
    debug!("{:?}", resolver.installs);
    for install in resolver.installs.iter() {
        let source = install.from.as_path();
        let destination = install.to.as_path();

        if can_install_file(source, destination) {
            trace!(
                "Copying file from {} to {}",
                source.display(),
                destination.display()
            );
            std::fs::create_dir_all(destination.parent().unwrap())?;
            std::fs::copy(source, destination)?;
        }
    }

    Ok(())
}

fn can_install_file(source: &Path, destination: &Path) -> bool {
    if !destination.exists() {
        return true;
    }

    let source_digest = smaug_lib::util::digest::file(source).unwrap();
    let destination_digest = smaug_lib::util::digest::file(destination).unwrap();
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

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use smaug::dragonruby;
use std::env;
use std::path::Path;

#[derive(Debug, Serialize, Display)]
#[display(fmt = "Opened docs in your web browser.")]
pub struct Docs;

#[derive(Debug, Display, Error, Serialize)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
    #[display(fmt = "Couldn't open your web browser.")]
    OpenError,
}

impl Command for Docs {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Docs Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let path = Path::new(directory);
        let path = std::fs::canonicalize(&path).expect("Could not find path");

        let config_path = path.join("Smaug.toml");

        let dragonruby = match smaug::config::load(&config_path) {
            Ok(config) => dragonruby::configured_version(&config),
            Err(..) => dragonruby::latest().ok(),
        };

        match dragonruby {
            None => Err(Box::new(Error::ConfiguredDragonRubyNotFound)),
            Some(dragonruby) => {
                let docs = dragonruby.path.join(dragonruby::dragonruby_docs_path());
                match open::that(docs) {
                    Ok(_) => Ok(Box::new(Docs {})),
                    Err(_) => Err(Box::new(Error::OpenError)),
                }
            }
        }
    }
}

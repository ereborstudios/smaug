use crate::command::CommandResult;
use crate::{command::Command, game_metadata};
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use smaug::dragonruby;
use std::env;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Run;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
}

impl Command for Run {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Run Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("PATH")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let path = Path::new(directory);

        let config_path = path.join("Smaug.toml");

        let config = smaug::config::load(&config_path)?;
        debug!("Smaug config: {:?}", config);

        trace!("Writing game metadata.");
        let metadata = game_metadata::from_config(&config);
        metadata
            .write(&path.join("metadata").join("game_metadata.txt"))
            .expect("Could not write game metadata.");

        let dragonruby = dragonruby::configured_version(&config);

        match dragonruby {
            None => Err(Box::new(Error::ConfiguredDragonRubyNotFound)),
            Some(dragonruby) => {
                let bin_dir = dragonruby.install_dir();

                debug!("DragonRuby Directory: {}", bin_dir.to_str().unwrap());
                let bin = bin_dir.join("dragonruby");

                trace!(
                    "Spawning Process {} {}",
                    bin.to_str().unwrap(),
                    path.to_str().unwrap()
                );
                process::Command::new(bin)
                    .arg(path)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                Ok(Box::new(format!(
                    "Successfully ran {}.",
                    config.project.unwrap().name
                )))
            }
        }
    }
}

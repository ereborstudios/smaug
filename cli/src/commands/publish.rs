use crate::command::CommandResult;
use crate::{command::Command, game_metadata};
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use smaug::dragonruby;
use smaug::util::dir::copy_directory;
use std::env;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Publish;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
}

impl Command for Publish {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Publish Command");

        let dragonruby_options: Vec<&str> = matches
            .values_of("DRAGONRUBY_ARGS")
            .unwrap_or_default()
            .collect();

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let path = Path::new(directory);
        let path = std::fs::canonicalize(&path).expect("Could not find path");

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
                let build_dir = bin_dir.join(path.file_name().unwrap());

                copy_directory(&path, &build_dir).expect("Could not copy to build directory.");

                let log_dir = build_dir.join("logs");
                let exception_dir = build_dir.join("exceptions");

                if log_dir.is_dir() {
                    std::fs::remove_dir_all(&log_dir).expect("couldn't remove logs");
                };

                if exception_dir.is_dir() {
                    std::fs::remove_dir_all(&exception_dir).expect("couldn't remove exceptions");
                };

                debug!("DragonRuby Directory: {}", bin_dir.to_str().unwrap());
                let bin = bin_dir.join("dragonruby-publish");

                trace!(
                    "Spawning Process {} {}",
                    bin.to_str().unwrap(),
                    path.to_str().unwrap()
                );
                process::Command::new(bin)
                    .current_dir(bin_dir.to_str().unwrap())
                    .arg(path.file_name().unwrap())
                    .args(dragonruby_options)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                copy_directory(&bin_dir.join("builds"), &path.join("builds"))
                    .expect("Could not copy builds.");

                let local_log_dir = &path.join("logs");
                if local_log_dir.is_dir() {
                    std::fs::remove_dir_all(&local_log_dir).expect("Couldn't remove local logs");
                }

                let local_exception_dir = &path.join("exceptions");
                if local_exception_dir.is_dir() {
                    std::fs::remove_dir_all(&local_exception_dir)
                        .expect("Couldn't remove local exceptions");
                }

                if log_dir.is_dir() {
                    smaug::util::dir::copy_directory(&log_dir, &local_log_dir)
                        .expect("couldn't copy logs");
                }

                if exception_dir.is_dir() {
                    smaug::util::dir::copy_directory(&exception_dir, &local_exception_dir)
                        .expect("couldn't copy exceptions");
                }

                std::fs::remove_dir_all(build_dir).expect("Could not clean up build dir");

                Ok(Box::new(format!(
                    "Successfully published {} to Itch.io!",
                    config.project.unwrap().name
                )))
            }
        }
    }
}

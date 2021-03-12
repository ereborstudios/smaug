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
pub struct Bind;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
    #[display(fmt = "The bind command is only available for DragonRuby pro.")]
    DragonRubyNotPro,
}

impl Command for Bind {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Bind Command");

        let output = matches.value_of("output").expect("No output found");
        let input = matches.value_of("FILE").expect("No input file found");

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

        let project_output = path.join(output);
        let project_input = path.join(input);

        trace!("Writing game metadata.");
        let metadata = game_metadata::from_config(&config);
        metadata
            .write(&path.join("metadata").join("game_metadata.txt"))
            .expect("Could not write game metadata.");

        let dragonruby = dragonruby::configured_version(&config);

        match dragonruby {
            None => Err(Box::new(Error::ConfiguredDragonRubyNotFound)),
            Some(dragonruby) => {
                if dragonruby.version.edition != dragonruby::Edition::Pro {
                    return Err(Box::new(Error::DragonRubyNotPro));
                }

                let bin_dir = dragonruby.install_dir();

                debug!("DragonRuby Directory: {}", bin_dir.to_str().unwrap());
                let bin = bin_dir.join(dragonruby::dragonruby_bind_name());

                trace!(
                    "Spawning Process {} --output={} {} {}",
                    bin.to_str().unwrap(),
                    project_output.display(),
                    project_input.display(),
                    dragonruby_options.join(" ")
                );
                process::Command::new(bin)
                    .current_dir(bin_dir.to_str().unwrap())
                    .arg(format!("--output={}", project_output.display()))
                    .arg(project_input)
                    .args(dragonruby_options)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                Ok(Box::new("Successfully created bindings."))
            }
        }
    }
}

use crate::command::CommandResult;
use crate::{command::Command, game_metadata};
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use smaug_lib::dragonruby;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use dunce;

#[derive(Debug, Display, Serialize)]
#[display(fmt = "Succesfully created bindings.")]
pub struct Bind;

#[derive(Debug, Display, Error, Serialize)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
    #[display(fmt = "The bind command is only available for DragonRuby pro.")]
    DragonRubyNotPro,
    #[display(fmt = "Couldn't load Smaug configuration.")]
    Config { path: PathBuf },
    #[display(fmt = "Could not find file at {}", "path.display()")]
    FileNotFound { path: PathBuf },
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
        let path = match dunce::canonicalize(directory) {
            Ok(dir) => dir,
            Err(..) => {
                return Err(Box::new(Error::FileNotFound {
                    path: Path::new(directory).to_path_buf(),
                }))
            }
        };

        let config_path = path.join("Smaug.toml");

        let config = match smaug_lib::config::load(&config_path) {
            Ok(config) => config,
            Err(..) => return Err(Box::new(Error::Config { path: config_path })),
        };

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

                let quiet = matches.is_present("json") || matches.is_present("quiet");

                let stdout = if quiet {
                    process::Stdio::null()
                } else {
                    process::Stdio::inherit()
                };

                process::Command::new(bin)
                    .current_dir(bin_dir.to_str().unwrap())
                    .arg(format!("--output={}", project_output.display()))
                    .arg(project_input)
                    .args(dragonruby_options)
                    .stdout(stdout)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                Ok(Box::new(Bind {}))
            }
        }
    }
}

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use std::env;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config;

#[derive(Debug, Serialize, Display)]
#[display(fmt = "{}", "print_config(&config)")]
pub struct ConfigResult {
    config: toml::Value,
}

#[derive(Debug, Display, Error, Serialize)]
enum Error {
    #[display(fmt = "Could not find Smaug.toml.")]
    ConfigNotFound { path: PathBuf },
    #[display(fmt = "Could not parse file at {}", "path.to_string_lossy()")]
    InvalidConfig { path: PathBuf },
}

impl Command for Config {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Config Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);

        let path = Path::new(directory);
        let config_file = path.join("Smaug.toml");

        if !config_file.is_file() {
            return Err(Box::new(Error::ConfigNotFound {
                path: path.to_path_buf(),
            }));
        }

        let contents = std::fs::read_to_string(config_file.clone()).unwrap();
        let loaded = toml::from_str::<toml::Value>(contents.as_str());

        match loaded {
            Ok(config) => Ok(Box::new(ConfigResult { config })),
            Err(..) => Err(Box::new(Error::InvalidConfig { path: config_file })),
        }
    }
}

fn print_config(config: &toml::Value) -> String {
    toml::to_string_pretty(config).expect("Couldn't serialize config")
}

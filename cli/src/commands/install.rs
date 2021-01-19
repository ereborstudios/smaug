use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use log::*;
use smaug::registry;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Install;

impl Command for Install {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Install Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("PATH")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let canonical = std::fs::canonicalize(directory)?;
        let path = Path::new(&canonical);

        let config_path = path.join("Smaug.toml");

        let config = smaug::config::load(&config_path)?;
        debug!("Smaug config: {:?}", config);

        let registry = registry::new_from_config(&config);

        match registry.install(path.join("smaug")) {
            Ok(()) => Ok(Box::new("Successfully installed your dependencies.")),
            Err(err) => Err(Box::new(err)),
        }
    }
}

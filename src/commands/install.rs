use crate::dragonruby;
use crate::lock::Lock;
use crate::project_config::ProjectConfig;
use log::*;
use std::env;
use std::path::Path;

pub fn call(matches: &clap::ArgMatches) {
    let current_directory = env::current_dir().unwrap();
    let filename: &str = matches
        .value_of("PATH")
        .unwrap_or_else(|| current_directory.to_str().unwrap());
    let path = Path::new(filename);
    debug!("Project Path: {}", path.to_str().unwrap());

    dragonruby::ensure_smaug_project(path);
    let config = ProjectConfig::load(path.join("Smaug.toml"));
    debug!("Smaug Configuration: {:?}", config);

    let lock = Lock::from_config(&config);
    debug!("Lock: {:?}", lock);
}

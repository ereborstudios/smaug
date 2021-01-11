use crate::dragonruby;
use log::*;
use std::fs;

pub fn call(_matches: &clap::ArgMatches) {
    let directory = dragonruby::dragonruby_directory();
    trace!("Removing directory {}", directory.to_str().unwrap());
    fs::remove_dir_all(directory).unwrap();
    info!("Uninstalled DragonRuby");
}

use crate::dragonruby;
use log::*;
use std::env;
use std::path::Path;
use std::process;

pub fn call(matches: &clap::ArgMatches) {
  dragonruby::ensure_installed();

  let current_directory = env::current_dir().unwrap();
  let filename: &str = matches
    .value_of("PATH")
    .unwrap_or(current_directory.to_str().unwrap());
  let path = Path::new(filename);
  debug!("Project Path: {}", path.to_str().unwrap());

  dragonruby::ensure_smaug_project(path);
  dragonruby::generate_metadata(path);

  let bin_dir = dragonruby::dragonruby_directory();
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
}

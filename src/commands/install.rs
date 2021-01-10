use crate::dragonruby;
use crate::git;
use crate::package_config::PackageConfig;
use crate::project_config::ProjectConfig;
use std::env;
use std::path::Path;

pub fn call(matches: &&clap::ArgMatches) {
  let current_directory = env::current_dir().unwrap();
  let filename: &str = matches
    .value_of("PATH")
    .unwrap_or(current_directory.to_str().unwrap());
  let path = Path::new(filename);

  dragonruby::ensure_smaug_project(path);
  let config = ProjectConfig::load(path.join("Smaug.toml"));

  for dependency in config.dependencies {
    if dependency.repo.is_some() {
      git::clone(&dependency);
    }
  }
}

fn copy_package(location: &Path) {
  let package = 
}

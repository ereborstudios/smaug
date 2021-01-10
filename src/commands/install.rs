use crate::dragonruby;
use crate::git;
use crate::project_config::ProjectConfig;
use crate::smaug;
use std::env;
use std::fs;
use std::path::Path;

pub fn call(matches: &&clap::ArgMatches) {
  let current_directory = env::current_dir().unwrap();
  let filename: &str = matches
    .value_of("PATH")
    .unwrap_or(current_directory.to_str().unwrap());
  let path = Path::new(filename);

  dragonruby::ensure_smaug_project(path);
  let config = ProjectConfig::load(path.clone().join("Smaug.toml"));

  for dependency in config.dependencies {
    let destination = smaug::cache_dir().join(dependency.name.as_ref().unwrap());

    if dependency.repo.is_some() {
      git::clone(&dependency, destination.clone());
    }

    copy_package(destination.clone().as_path(), &path);
  }
}

fn copy_package(package: &Path, project: &Path) {
  let package_project = ProjectConfig::load(package.join("Smaug.toml"));

  for file in package_project.files {
    let source = package.join(file.from);
    let destination = project.join(file.to);

    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::copy(source, destination).unwrap();
  }
}

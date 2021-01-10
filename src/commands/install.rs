use crate::dependency::Dependency;
use crate::dragonruby;
use crate::git;
use crate::project_config::ProjectConfig;
use crate::smaug;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn call(matches: &&clap::ArgMatches) {
  let current_directory = env::current_dir().unwrap();
  let filename: &str = matches
    .value_of("PATH")
    .unwrap_or(current_directory.to_str().unwrap());
  let path = Path::new(filename);

  dragonruby::ensure_smaug_project(path);
  let config = ProjectConfig::load(path.clone().join("Smaug.toml"));

  for dependency in config.dependencies {
    let cache_dir = smaug::cache_dir();

    match Dependency::from_config(&dependency) {
      Some(Dependency::Git { repo, branch }) => {
        let clone = git::Clone { repo, branch };
        let destination = cache_dir.join(dependency.name.as_ref().unwrap());
        clone.clone(&destination);
        let source = destination.clone();
        copy_package(&source, &path);
      }
      Some(Dependency::Dir { path: dir }) => {
        let source = dir.to_path_buf();
        copy_package(&source, &path);
      }
      None => {
        println!("Malformed dependency: {}", dependency.name.unwrap());
        process::exit(exitcode::DATAERR);
      }
      _ => {
        println!("Not implemented yet: {:?}", dependency);
        process::exit(exitcode::TEMPFAIL);
      }
    }
  }
}

fn copy_package(package: &Path, project: &Path) {
  let package_project = ProjectConfig::load(package.join("Smaug.toml"));
  println!("{:?}", package_project);

  for file in package_project.files {
    let source = package.join(file.from);
    let destination = project.join(file.to);

    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::copy(source, destination).unwrap();
  }
}

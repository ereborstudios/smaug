use crate::dragonruby;
use crate::smaug;
use log::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use walkdir::WalkDir;

pub fn call(matches: &clap::ArgMatches) {
  dragonruby::ensure_installed();

  let path = matches.value_of("PATH").unwrap();
  let destination = Path::new(path);
  debug!("Project path: {}", destination.to_str().unwrap());

  if destination.exists() {
    smaug::print_error(format!("{} already exists", path));
    process::exit(exitcode::DATAERR);
  }

  trace!("Creating directory {}", destination.to_str().unwrap());
  fs::create_dir(destination).unwrap();

  let template = dragonruby::dragonruby_directory().join("mygame");
  debug!("Template Directory: {}", template.to_str().unwrap());
  copy_directory(template, destination.to_path_buf());
}

fn copy_directory(source: PathBuf, destination: PathBuf) {
  for entry in WalkDir::new(source.clone()) {
    let entry = entry.unwrap();
    let base = entry.path().strip_prefix(source.clone()).unwrap();
    let file_source = source.clone().join(base);
    let file_destination = destination.clone().join(base);

    if file_source.is_file() {
      let directory = file_destination.parent().unwrap();
      trace!(
        "Copying file from {} to {}",
        file_source.to_str().unwrap(),
        file_destination.to_str().unwrap()
      );
      fs::create_dir_all(directory).unwrap();
      fs::copy(file_source, file_destination).unwrap();
    }
  }
}

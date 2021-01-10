use crate::dragonruby;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use walkdir::WalkDir;

pub fn call(matches: &&clap::ArgMatches) {
  dragonruby::ensure_installed();

  let path = matches.value_of("PATH").unwrap();
  let destination = Path::new(path);

  if destination.exists() {
    println!("{} already exists", path);
    process::exit(exitcode::DATAERR);
  }

  fs::create_dir(destination).unwrap();

  let template = dragonruby::dragonruby_directory().join("mygame");
  copy_directory(template, destination.to_path_buf());
}

fn copy_directory(source: PathBuf, destination: PathBuf) {
  for entry in WalkDir::new(source.clone()) {
    let entry = entry.unwrap();
    let base = entry.path().strip_prefix(source.clone()).unwrap();
    let file_source = source.clone().join(base);
    let file_destination = destination.clone().join(base);

    if file_source.is_file() {
      fs::create_dir_all(file_destination.parent().unwrap()).unwrap();
      fs::copy(file_source, file_destination).unwrap();
    }
  }
}

use crate::dragonruby;
use log::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use zip_extensions::is_zip;
use zip_extensions::zip_extract;

pub fn call(matches: &clap::ArgMatches) {
  let filename: &str = matches.value_of("FILE").unwrap();
  let path = Path::new(filename);
  let destination: PathBuf;

  if path.exists() && is_zip(&path.to_path_buf()) {
    destination = setup_destination();
    debug!("Project Path: {}", destination.to_str().unwrap());
    extract(path, &destination);
  } else {
    error!("The file {} does not exist", path.to_str().unwrap());
    process::exit(exitcode::NOINPUT);
  }
}

fn setup_destination() -> PathBuf {
  let path = dragonruby::dragonruby_directory();
  let destination = Path::parent(&path).unwrap();

  trace!("Creating directory {}", destination.to_str().unwrap());
  let result = fs::create_dir_all(destination)
    .and_then(|()| fs::remove_dir_all(destination).and_then(|()| fs::create_dir_all(destination)));

  match result {
    Ok(()) => return destination.to_path_buf(),
    Err(error) => {
      error!(
        "Could not create directory at {}\n{}",
        destination.to_str().unwrap(),
        error
      );
      process::exit(exitcode::DATAERR);
    }
  }
}

fn extract(source: &Path, destination: &Path) {
  info!("Installing DragonRuby from {}", source.to_str().unwrap());
  trace!("Extracting Dragonruby from {}", source.to_str().unwrap());

  zip_extract(&source.to_path_buf(), &destination.to_path_buf()).unwrap();
}

use crate::dragonruby;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use zip_extensions::zip_extract;

pub fn call(matches: &&clap::ArgMatches) {
  let filename: &str = matches.value_of("FILE").unwrap();
  let path = Path::new(filename);
  let destination: PathBuf;

  if path.exists() {
    destination = setup_destination();
    extract(path, &destination);
  } else {
    println!("The file {} does not exist", path.to_str().unwrap());
    process::exit(exitcode::NOINPUT);
  }
}

fn setup_destination() -> PathBuf {
  let path = dragonruby::dragonruby_directory();
  let destination = Path::parent(&path).unwrap();

  let result = fs::create_dir_all(destination)
    .and_then(|()| fs::remove_dir_all(destination).and_then(|()| fs::create_dir_all(destination)));

  match result {
    Ok(()) => return destination.to_path_buf(),
    Err(error) => {
      println!(
        "Error creating directory at {}\n{}",
        destination.to_str().unwrap(),
        error
      );
      process::exit(exitcode::DATAERR);
    }
  }
}

fn extract(source: &Path, destination: &Path) {
  println!("Installing DragonRuby from {}", source.to_str().unwrap());

  zip_extract(&source.to_path_buf(), &destination.to_path_buf()).unwrap();
}

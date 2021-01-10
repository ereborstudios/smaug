use crate::dragonruby;
use std::env;
use std::fs;
use std::include_str;
use std::path::Path;
use std::process;

pub fn call(matches: &&clap::ArgMatches) {
  dragonruby::ensure_installed();

  let current_directory = env::current_dir().unwrap();
  let directory: &str = matches
    .value_of("PATH")
    .unwrap_or(current_directory.to_str().unwrap());
  let pathbuf = Path::new(directory).join("Smaug.toml");
  let path = pathbuf.as_path();

  if path.exists() {
    println!(
      "{} is already a Smaug project.",
      path.parent().unwrap().display()
    );
    process::exit(exitcode::USAGE);
  }

  generate_config(&path);
}

fn generate_config(path: &Path) {
  let config = include_str!("../../data/Smaug.toml");

  fs::write(path, config).unwrap();
}
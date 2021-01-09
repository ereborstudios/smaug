use crate::dragonruby;
use std::fs;

pub fn uninstall(_matches: &&clap::ArgMatches) {
  fs::remove_dir_all(dragonruby::dragonruby_directory()).unwrap();
}

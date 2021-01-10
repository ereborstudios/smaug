use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use url::Url;

pub fn download(url: &Url, destination: &Path) {
  if destination.is_file() {
    fs::remove_file(destination).unwrap();
  }

  fs::create_dir_all(destination.parent().unwrap()).unwrap();
  let mut file = File::create(destination).unwrap();

  let mut response = reqwest::blocking::get(url.as_str()).unwrap();

  io::copy(&mut response, &mut file).unwrap();
}

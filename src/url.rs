use log::*;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn download(url: &str, destination: &Path) {
    if destination.is_file() {
        trace!("Removing file {}", destination.to_str().unwrap());
        fs::remove_file(destination).unwrap();
    }

    trace!(
        "Creating directory {}",
        destination.parent().and_then(|p| p.to_str()).unwrap()
    );
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    let mut file = File::create(destination).unwrap();

    let mut response = reqwest::blocking::get(url).unwrap();

    trace!("Downloading file to {}", destination.to_str().unwrap());
    io::copy(&mut response, &mut file).unwrap();
}

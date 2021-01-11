use crate::dragonruby;
use crate::smaug;
use log::*;
use std::env;
use std::fs;
use std::include_str;
use std::path::Path;
use std::process;

pub fn call(matches: &clap::ArgMatches) {
    dragonruby::ensure_installed();

    let current_directory = env::current_dir().unwrap();
    let directory: &str = matches
        .value_of("PATH")
        .unwrap_or_else(|| current_directory.to_str().unwrap());
    debug!("Directory: {}", directory);
    let pathbuf = Path::new(directory).join("Smaug.toml");
    let path = pathbuf.as_path();
    debug!("Smaug Configuration: {}", path.to_str().unwrap());

    if path.exists() {
        smaug::print_error(format!(
            "{} is already a Smaug project.",
            path.parent().unwrap().display(),
        ));
        process::exit(exitcode::USAGE);
    }

    generate_config(&path);
    info!("Created Smaug.toml edit the values with your package's information.");
    scrawl::editor::new()
        .file(path.to_str().unwrap())
        .edit()
        .open()
        .unwrap();
}

pub fn generate_config(path: &Path) {
    let config = include_str!("../../data/Package.toml");

    trace!("Writing Smaug configuration to {}", path.to_str().unwrap());
    fs::write(path, config).unwrap();
}
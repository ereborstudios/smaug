use crate::dragonruby;
use crate::utils::copy_directory;
use log::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn call(matches: &clap::ArgMatches) {
    dragonruby::ensure_installed();

    let current_directory = env::current_dir().unwrap();
    let filename: &str = matches
        .value_of("PATH")
        .unwrap_or_else(|| current_directory.to_str().unwrap());
    let path = Path::new(filename);
    debug!("Project Path: {}", path.to_str().unwrap());

    dragonruby::ensure_smaug_project(path);
    dragonruby::generate_metadata(path);

    let dragonruby_directory = dragonruby::dragonruby_directory();
    debug!(
        "DragonRuby Directory: {}",
        dragonruby_directory.to_str().unwrap()
    );

    let build_directory = dragonruby_directory.join(path.file_name().unwrap());
    copy_directory(&path, &build_directory);

    let bin = dragonruby_directory.join("dragonruby-publish");

    trace!(
        "Spawning Process {} {} {}",
        bin.to_str().unwrap(),
        "--only-package",
        path.to_str().unwrap(),
    );
    process::Command::new(bin)
        .current_dir(dragonruby_directory.to_str().unwrap())
        .arg(path.file_name().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    trace!("Removing directory {}", build_directory.to_str().unwrap());
    fs::remove_dir_all(build_directory).unwrap();

    let builds_directory = dragonruby_directory.join("builds");
    let new_builds_directory = path.join("builds");
    copy_directory(&builds_directory, &new_builds_directory);

    trace!("Removing directory {}", builds_directory.to_str().unwrap());
    fs::remove_dir_all(builds_directory).unwrap();
}

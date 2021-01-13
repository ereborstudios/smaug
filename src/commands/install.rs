use crate::dragonruby;
use crate::lock::Lock;
use crate::project_config::ProjectConfig;
use crate::smaug;
use crate::utils::copy_directory;
use log::*;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;

pub fn call(matches: &clap::ArgMatches) -> io::Result<()> {
    let current_directory = env::current_dir().unwrap();
    let filename: &str = matches
        .value_of("PATH")
        .unwrap_or_else(|| current_directory.to_str().unwrap());
    let path = Path::new(filename);
    debug!("Project Path: {}", path.to_str().unwrap());

    dragonruby::ensure_smaug_project(path);

    install_from_config(&path)?;
    Ok(())
}

fn install_from_config(path: &Path) -> io::Result<()> {
    let config = ProjectConfig::load(&path.join("Smaug.toml")).unwrap();
    debug!("Smaug Configuration: {:?}", config);

    let lock_file = path.join("Smaug.lock");
    debug!("Lock file: {:?}", lock_file);
    match Lock::load(&lock_file) {
        Err(..) => {
            smaug::print_error("Could not parse Smaug.lock");
            exit(exitcode::DATAERR);
        }
        Ok(lock) => {
            let updated_lock = lock.update_from_config(&config).unwrap();
            install_packages(&updated_lock, &path)?;
            remove_packages(&updated_lock, &path)?;
            create_index(&updated_lock, &path)?;
            write_lock(&updated_lock, &path)?;
        }
    }

    info!("Dependencies installed. Add `require \"smaug.rb\"` to the top of your main.rb.");
    Ok(())
}

fn install_packages(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Installing packages");

    for package in lock.packages.iter() {
        let destination = path.join("smaug").join(package.name.clone());

        if destination.exists() {
            debug!("Dependency {} already installed.", package.name.clone());
        } else {
            trace!("Installing {}", package.name.clone());
            let source = package.cache.clone().unwrap();

            trace!(
                "Copying from {} to {}",
                source.to_str().unwrap(),
                destination.to_str().unwrap()
            );
            copy_directory(&source, &destination)?;
        }
    }

    trace!("Removing cache");
    let cachedir = smaug::cache_dir();

    if cachedir.exists() {
        fs::remove_dir_all(smaug::cache_dir())?;
    }

    Ok(())
}

fn remove_packages(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Removing packages");

    for package in lock.removed_packages.iter() {
        let destination = path.join(package);
        trace!("Removing package {}", package);
        fs::remove_dir_all(destination)?;
    }

    Ok(())
}

fn create_index(lock: &Lock, path: &Path) -> io::Result<()> {
    let mut index = String::new();
    index.push_str("# This file was automatically @generated by Smaug.\n");
    index.push_str("# It is recommended not to manually edit this file.\n\n");

    for package in lock.packages.iter() {
        let destination = path.join("smaug").join(package.name.clone());

        if destination.exists() {
            for require in package.requires.iter() {
                let require_path = format!("smaug/{}/{}", package.name.clone(), require);
                index.push_str(format!("require \"{}\"\n", require_path).as_str());
            }
        }
    }

    let index_file = path.join("smaug.rb");
    fs::write(index_file, index)?;
    Ok(())
}

fn write_lock(lock: &Lock, path: &Path) -> io::Result<()> {
    let destination = path.join("Smaug.lock");

    match toml::to_string(lock) {
        Ok(content) => fs::write(destination, content)?,
        Err(message) => {
            let message = format!("Failed to write lock file: {}", message);
            smaug::print_error(message);
            exit(exitcode::TEMPFAIL);
        }
    }

    Ok(())
}

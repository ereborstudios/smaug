use crate::dependency::Dependency;
use crate::digest;
use crate::dragonruby;
use crate::lock;
use crate::lock::Lock;
use crate::project_config;
use crate::project_config::ProjectConfig;
use crate::smaug;
use log::*;
use question::{Answer, Question};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

pub fn call(matches: &clap::ArgMatches) -> io::Result<()> {
    let current_directory = env::current_dir().unwrap();
    let filename: &str = matches
        .value_of("PATH")
        .unwrap_or_else(|| current_directory.to_str().unwrap());
    let path = Path::new(filename);
    debug!("Project Path: {}", path.to_str().unwrap());

    dragonruby::ensure_smaug_project(path);

    if matches.is_present("package") {
        println!("Installing Individual Package");
        println!("{:?}", matches.value_of("package"));
        install_package(&path, matches.value_of("package").unwrap())?;
    } else {
        install_from_config(&path)?;
    }

    Ok(())
}

fn install_package(path: &Path, package: &str) -> io::Result<()> {
    let dependency_config = project_config::load_dependency_string("temp", package);
    let dependency = Dependency::from_config(&dependency_config).unwrap();

    let dependency_path = dependency.cache();
    let file_lock = lock::parse_package(&dependency_path, dependency.name);
    let lock = Lock { files: file_lock };

    install_packages(&lock, &path)?;
    Ok(())
}

fn install_from_config(path: &Path) -> io::Result<()> {
    let config = ProjectConfig::load(path.join("Smaug.toml"));
    debug!("Smaug Configuration: {:?}", config);

    let lock_result = Lock::from_config(&config);
    debug!("Lock: {:?}", lock_result);

    match lock_result {
        Ok(lock) => {
            install_packages(&lock, &path)?;
            remove_packages(&lock, &path)?;
            create_index(&lock, &path)?;
            create_lock_file(&lock, &path)?;
        }
        Err(error) => {
            smaug::print_error(format!("Lock file error: {:?}", error));
            process::exit(exitcode::DATAERR);
        }
    }

    Ok(())
}

fn install_packages(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Installing packages");
    let previous_lock = read_lock_file(path);
    debug!("Existing Lock: {:?}", previous_lock);

    let mut files_to_install = HashSet::new();
    let mut changed_files = HashSet::new();

    for file in lock.files.clone() {
        let destination = path.join(file.destination.clone());
        let destination_string = String::from(destination.clone().to_str().unwrap());
        files_to_install.insert(destination_string);
    }

    if let Some(previous_lock_file) = previous_lock {
        for file in previous_lock_file.files.clone() {
            let destination = path.join(file.destination.clone());
            let destination_string = String::from(destination.clone().to_str().unwrap());
            if destination.exists() {
                let digest = digest::file(&destination.clone()).unwrap();

                if !digest.eq(&file.digest) {
                    changed_files.insert(destination_string);
                }
            }
        }
    }

    for file in lock.files.iter() {
        let destination = path.join(file.destination.clone());
        let destination_string = String::from(destination.clone().to_str().unwrap());

        if changed_files.contains(&destination_string.clone()) {
            let changed_path = destination_string.replace(path.to_str().unwrap(), "");
            let changed_path = changed_path.replacen('/', "", 1);

            let question = format!(
                "{} has changed since the last install. Do you want to overwrite it?",
                changed_path
            );

            let answer = Question::new(question.as_str())
                .default(Answer::YES)
                .show_defaults()
                .confirm();

            if answer == Answer::YES {
                copy_file(&file.clone().source.unwrap(), &destination)?;
            }
        } else {
            copy_file(&file.clone().source.unwrap(), &destination)?;
        }
    }
    Ok(())
}

fn remove_packages(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Removing unused packages");
    let previous_lock = read_lock_file(path);
    debug!("Existing Lock: {:?}", previous_lock);

    let mut deleted_files = HashMap::new();
    let mut files_to_install = HashSet::new();

    for file in lock.files.clone() {
        let destination = path.join(file.destination.clone());
        let destination_string = String::from(destination.clone().to_str().unwrap());
        files_to_install.insert(destination_string);
    }

    if let Some(previous_lock_file) = previous_lock {
        for file in previous_lock_file.files.clone() {
            let destination = path.join(file.destination.clone());
            let destination_string = String::from(destination.clone().to_str().unwrap());

            if !files_to_install.contains(&destination_string) {
                deleted_files.insert(destination_string, file);
            }
        }
    }

    for (.., file) in deleted_files {
        let destination = path.join(file.destination.clone());
        let destination_string = String::from(destination.clone().to_str().unwrap());
        let digest = digest::file(&destination).unwrap();

        if destination.exists() && !digest.eq(&file.digest) {
            let changed_path = destination_string.replace(path.to_str().unwrap(), "");
            let changed_path = changed_path.replacen('/', "", 1);

            let question = format!(
                "{} has changed since the last install. Do you want to delete it?",
                changed_path
            );

            let answer = Question::new(question.as_str())
                .default(Answer::YES)
                .show_defaults()
                .confirm();

            if answer == Answer::YES {
                trace!("Removing file {}", destination.to_str().unwrap());
                fs::remove_file(destination)?;
            }
        } else {
            fs::remove_file(destination)?;
        }
    }

    Ok(())
}

fn copy_file(source: &Path, destination: &Path) -> io::Result<()> {
    let directory = destination.parent().unwrap();
    trace!("Creating directory {}", directory.to_str().unwrap());
    fs::create_dir_all(directory)?;

    trace!(
        "Copying file from {} to {}",
        source.to_str().unwrap(),
        destination.to_str().unwrap()
    );
    fs::copy(source, destination)?;
    Ok(())
}

fn create_index(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Creating file index");
    let mut index = String::new();
    index.push_str("# This file is automatically @generated by Smaug.\n");
    index.push_str("# Do not edit it manually.\n\n");

    for file in lock.files.iter() {
        if file.require {
            let destination = file.destination.to_str().unwrap();
            let require = format!("require \"{}\"\n", destination);
            index.push_str(require.as_str());
        }
    }

    let index_file = path.join("app/smaug.rb");
    fs::write(index_file, index)?;
    Ok(())
}

fn create_lock_file(lock: &Lock, path: &Path) -> io::Result<()> {
    trace!("Creating Smaug.lock");
    let lock_file = path.join("Smaug.lock");
    let lock_contents = toml::to_string(&lock).unwrap();

    fs::write(lock_file, lock_contents)?;
    Ok(())
}

fn read_lock_file(path: &Path) -> Option<Lock> {
    let file = path.join("Smaug.lock");

    if file.exists() {
        let contents = fs::read_to_string(file).unwrap();
        let lock: Lock = toml::from_str(&contents).unwrap();

        Some(lock)
    } else {
        None
    }
}

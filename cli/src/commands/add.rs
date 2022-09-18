use crate::command::Command;
use crate::command::CommandResult;
use crate::commands::install::Install;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::{value, Document};
use dunce;

pub struct Add;

#[derive(Debug, Display, Error, Serialize)]
pub enum Error {
    #[display(fmt = "Could not find Smaug.toml at {}", "path.display()")]
    FileNotFound { path: PathBuf },
    #[display(fmt = "{} has already been added to this project.", "name")]
    AlreadyAdded { name: String },
    #[display(fmt = "Could not fetch from registry.")]
    Registry,
    #[display(fmt = "Could not install packages.")]
    Install,
}

#[derive(Debug, Display, Serialize)]
#[display(fmt = "Added {} {} to your project.", "package", "version")]
pub struct AddResult {
    package: String,
    version: String,
}

impl Command for Add {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Add Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());

        debug!("Directory: {}", directory);

        let canonical = match dunce::canonicalize(directory) {
            Ok(dir) => dir,
            Err(..) => {
                return Err(Box::new(Error::FileNotFound {
                    path: Path::new(directory).to_path_buf(),
                }))
            }
        };

        let path = Path::new(&canonical);
        let path = dunce::canonicalize(&path).expect("Could not find path");

        let config_path = path.join("Smaug.toml");

        if !config_path.is_file() {
            return Err(Box::new(Error::FileNotFound { path: config_path }));
        }

        let config =
            std::fs::read_to_string(config_path.clone()).expect("Could not read Smaug.toml");

        let package_name = matches.value_of("PACKAGE").expect("No package given");
        let latest_version = match fetch_from_registry(package_name.to_string()) {
            Ok(version) => version,
            Err(..) => return Err(Box::new(Error::Registry)),
        };

        trace!("Latest version: {}", latest_version);

        let mut doc = config.parse::<Document>().expect("invalid doc");

        {
            let dependencies = doc["dependencies"].as_table().expect("No dependencies");

            debug!("Dependencies: {:?}", dependencies);

            if dependencies.contains_key(package_name) {
                return Err(Box::new(Error::AlreadyAdded {
                    name: package_name.to_string(),
                }));
            }
        }

        doc["dependencies"][package_name] = value(latest_version.clone());

        std::fs::write(config_path, doc.to_string_in_original_order())
            .expect("Couldn't write config file.");

        if Install.run(matches).is_err() {
            return Err(Box::new(Error::Install));
        }

        Ok(Box::new(AddResult {
            package: package_name.to_string(),
            version: latest_version,
        }))
    }
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    version: String,
}

#[derive(Debug, Deserialize)]
struct PackageResponse {
    version: VersionResponse,
}

fn fetch_from_registry(name: String) -> std::io::Result<String> {
    let url = format!("https://api.smaug.dev/packages/{}", name);
    trace!("Fetching latest version from {}", url);

    let response = reqwest::blocking::get(url.as_str());

    match response {
        Err(..) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find package",
        )),
        Ok(response) => parse_response(response, name),
    }
}

fn parse_response(response: reqwest::blocking::Response, name: String) -> std::io::Result<String> {
    if response.status().is_success() {
        let package_response: PackageResponse =
            response.json().expect("Couldn't parse registry response");

        Ok(package_response.version.version)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Couldn't fetch {} from repository", name),
        ))
    }
}

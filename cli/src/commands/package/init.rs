use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

pub struct Init;

#[derive(Serialize)]
struct PackageConfig {
    version: String,
    edition: String,
    name: String,
}

#[derive(Debug, Display, Serialize)]
#[display(fmt = "Initialized your DragonRuby Package")]
pub struct InitResult {
    path: PathBuf,
}

#[derive(Debug, Display, Error, Serialize)]
enum Error {
    #[display(fmt = "DragonRuby is not installed. See smaug dragonruby help install for details.")]
    DragonRubyNotFound,
}

static TEMPLATE: &str = include_str!("../../../templates/Package.toml.template");

impl Command for Init {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Init Command");

        let latest = smaug::dragonruby::latest();
        if let Err(..) = latest {
            return Err(Box::new(Error::DragonRubyNotFound {}));
        }
        let latest = latest.unwrap();

        debug!("Latest DragonRuby: {}", latest);

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("PATH")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let path = Path::new(directory).canonicalize().unwrap();

        let mut tt = TinyTemplate::new();
        tt.add_template("Project.toml", TEMPLATE)
            .expect("couldn't add template.");

        let version = latest.version;
        let edition = match version.edition {
            smaug::dragonruby::Edition::Standard => "standard",
            smaug::dragonruby::Edition::Pro => "pro",
        };

        let context = PackageConfig {
            name: path
                .file_name()
                .expect("directory has no file name.")
                .to_string_lossy()
                .to_string(),
            version: format!("{}.{}", version.version.major, version.version.minor),
            edition: edition.to_string(),
        };

        let rendered = tt
            .render("Project.toml", &context)
            .expect("Could not render Project.toml");

        let config_path = path.join("Smaug.toml");

        trace!("Writing configuration to {}", config_path.display());
        std::fs::write(config_path, rendered).expect("Could not write file");

        Ok(Box::new(InitResult { path }))
    }
}

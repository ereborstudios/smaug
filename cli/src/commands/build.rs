use crate::command::CommandResult;
use crate::{command::Command, game_metadata};
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use smaug::dragonruby;
use smaug::util::dir::copy_directory;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
pub struct Build;

#[derive(Debug, Serialize, Display)]
#[display(fmt = "Successfully built {}", "project_name")]
pub struct BuildResult {
    project_name: String,
}
#[derive(Debug, Display, Error, Serialize)]
pub enum Error {
    #[display(
        fmt = "Could not find the configured version of DragonRuby. Install it with `smaug dragonruby install`"
    )]
    ConfiguredDragonRubyNotFound,
    #[display(fmt = "Couldn't load Smaug configuration.")]
    ConfigError { path: PathBuf },
    #[display(fmt = "Could not find file at {}", "path.display()")]
    FileNotFound { path: PathBuf },
}

impl Command for Build {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Build Command");

        let dragonruby_options: Vec<&str> = matches
            .values_of("DRAGONRUBY_ARGS")
            .unwrap_or_default()
            .collect();

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let path = match std::fs::canonicalize(directory) {
            Ok(dir) => dir,
            Err(..) => {
                return Err(Box::new(Error::FileNotFound {
                    path: Path::new(directory).to_path_buf(),
                }))
            }
        };

        let config_path = path.join("Smaug.toml");

        let config = match smaug::config::load(&config_path) {
            Ok(conf) => conf,
            Err(..) => return Err(Box::new(Error::ConfigError { path: config_path })),
        };
        debug!("Smaug config: {:?}", config);

        trace!("Writing game metadata.");
        let metadata = game_metadata::from_config(&config);
        metadata
            .write(&path.join("metadata").join("game_metadata.txt"))
            .expect("Could not write game metadata.");

        let dragonruby = dragonruby::configured_version(&config);

        match dragonruby {
            None => Err(Box::new(Error::ConfiguredDragonRubyNotFound)),
            Some(dragonruby) => {
                let bin_dir = dragonruby.install_dir();
                let build_dir = bin_dir.join(path.file_name().unwrap());

                copy_directory(&path, &build_dir).expect("Could not copy to build directory.");

                let log_dir = build_dir.join("logs");
                let exception_dir = build_dir.join("exceptions");

                rm_rf::ensure_removed(&log_dir).expect("couldn't remove logs");
                rm_rf::ensure_removed(&exception_dir).expect("couldn't remove exceptions");

                debug!("DragonRuby Directory: {}", bin_dir.to_str().unwrap());
                let bin = bin_dir.join(dragonruby::dragonruby_publish_name());

                trace!(
                    "Spawning Process {} {} {} {}",
                    bin.to_str().unwrap(),
                    "--only-package",
                    path.to_str().unwrap(),
                    dragonruby_options.join(" "),
                );

                let quiet = matches.is_present("json") || matches.is_present("quiet");

                let stdout = if quiet {
                    process::Stdio::null()
                } else {
                    process::Stdio::inherit()
                };

                process::Command::new(bin)
                    .current_dir(bin_dir.to_str().unwrap())
                    .arg("--only-package")
                    .args(dragonruby_options)
                    .arg(path.file_name().unwrap())
                    .stdout(stdout)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                copy_directory(&bin_dir.join("builds"), &path.join("builds"))
                    .expect("Could not copy builds.");

                rm_rf::ensure_removed(build_dir).expect("Could not clean up build dir");

                let local_log_dir = &path.join("logs");
                rm_rf::ensure_removed(&local_log_dir).expect("Couldn't remove local logs");

                let local_exception_dir = &path.join("exceptions");
                rm_rf::ensure_removed(&local_exception_dir)
                    .expect("Couldn't remove local exceptions");

                if log_dir.is_dir() {
                    smaug::util::dir::copy_directory(&log_dir, &local_log_dir)
                        .expect("couldn't copy logs");
                }

                if exception_dir.is_dir() {
                    smaug::util::dir::copy_directory(&exception_dir, &local_exception_dir)
                        .expect("couldn't copy exceptions");
                }

                Ok(Box::new(BuildResult {
                    project_name: config.project.unwrap().name,
                }))
            }
        }
    }
}

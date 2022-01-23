use crate::command::Command;
use crate::command::CommandResult;
use ::smaug_lib::dragonruby;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use smaug_lib::smaug;

#[derive(Debug)]
pub struct Uninstall;

#[derive(Debug, Display, Serialize)]
#[display(fmt = "Uninstalled DragonRuby {}", "version")]
pub struct UninstallResult {
    version: String,
}

#[derive(Debug, Display, Error, Serialize)]
enum Error {
    #[display(fmt = "Could not find DragonRuby {}", "version")]
    DragonRubyNotFound { version: String },
}

impl Command for Uninstall {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Uninstall Command");
        let version = matches.value_of("VERSION").expect("No version specified");

        let location = smaug::data_dir().join("dragonruby").join(version);
        let dragonruby = dragonruby::new(&location);

        match dragonruby {
            Ok(dr) => {
                rm_rf::ensure_removed(dr.path).expect("DragonRuby folder not removed.");

                Ok(Box::new(UninstallResult {
                    version: version.to_string(),
                }))
            }
            Err(..) => Err(Box::new(Error::DragonRubyNotFound {
                version: version.to_string(),
            })),
        }
    }
}

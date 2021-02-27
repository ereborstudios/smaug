use crate::command::Command;
use crate::command::CommandResult;
use ::smaug::dragonruby;
use clap::ArgMatches;
use log::*;
use smaug::smaug;

use super::list::List;

#[derive(Debug)]
pub struct Uninstall;

impl Command for Uninstall {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Uninstall Command");
        let version = matches.value_of("VERSION");

        match version {
            None => {
                let list = List.run(matches);

                match list {
                    Ok(message) => Ok(Box::new(format!(
                        "smaug dragonruby list <VERSION>\n\nHere are the installed versions:\n\n{}",
                        message
                    ))),
                    Err(..) => list,
                }
            }
            Some(version) => {
                let location = smaug::data_dir().join("dragonruby").join(version);
                let dragonruby = dragonruby::new(&location);

                match dragonruby {
                    Ok(dr) => {
                        rm_rf::ensure_removed(dr.path).expect("DragonRuby folder not removed.");
                        let message = format!("Uninstalled {}", dr.version);

                        Ok(Box::new(message))
                    }
                    Err(..) => {
                        let list = List.run(matches);

                        match list {
                            Ok(message) => Ok(Box::new(format!(
                        "DragonRuby version \"{}\" not found. Here are the installed versions:\n\n{}",
                        version,
                        message
                    ))),
                            Err(..) => list,
                        }
                    }
                }
            }
        }
    }
}

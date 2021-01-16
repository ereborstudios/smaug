pub mod install;
pub mod list;
pub mod uninstall;

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use install::Install;
use list::List;
use log::*;
use uninstall::Uninstall;

#[derive(Debug)]
pub struct DragonRuby;

impl Command for DragonRuby {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Dragon Ruby Command");

        let command: Box<dyn Command> = match matches.subcommand_name() {
            Some("install") => Box::new(Install),
            Some("list") => Box::new(List),
            Some("uninstall") => Box::new(Uninstall),
            _ => unreachable!(),
        };

        let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

        command.run(&subcommand_matches.unwrap())
    }
}

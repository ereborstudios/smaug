pub mod install;

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use install::Install;
use log::*;

#[derive(Debug)]
pub struct DragonRuby;

impl Command for DragonRuby {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Dragon Ruby Command");

        let command: Box<dyn Command> = match matches.subcommand_name() {
            Some("install") => Box::new(Install),
            _ => unreachable!(),
        };

        let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

        command.run(&subcommand_matches.unwrap())
    }
}

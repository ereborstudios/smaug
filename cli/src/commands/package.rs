use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use log::*;

pub mod init;

#[derive(Debug)]
pub struct Package;

impl Command for Package {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Package Command");

        let command: Box<dyn Command> = match matches.subcommand_name() {
            Some("init") => Box::new(init::Init),
            _ => unreachable!(),
        };

        let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

        command.run(&subcommand_matches.unwrap())
    }
}

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
        let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

        match matches.subcommand_name() {
            Some("init") => init::Init.run(&subcommand_matches.unwrap()),
            _ => unreachable!(),
        }
    }
}

pub mod install;
pub mod list;
pub mod uninstall;

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use install::Install;
use list::List;
use log::*;
use serde::Serialize;
use std::fmt::Display;
use uninstall::Uninstall;

#[derive(Debug)]
pub struct DragonRuby;

trait Result: Display + Serialize {}

impl Command for DragonRuby {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Dragon Ruby Command");

        let subcommand_matches = matches
            .subcommand_matches(matches.subcommand_name().unwrap())
            .unwrap();

        match matches.subcommand_name() {
            Some("install") => Install.run(&subcommand_matches),
            Some("list") => List.run(&subcommand_matches),
            Some("uninstall") => Uninstall.run(&subcommand_matches),
            _ => unreachable!(),
        }
    }
}

use clap::ArgMatches;
use std::error::Error;
use std::fmt::Display;

pub type CommandResult = Result<Box<dyn Display>, Box<dyn Error>>;
pub trait Command {
    fn run(&self, matches: &ArgMatches) -> CommandResult;
}

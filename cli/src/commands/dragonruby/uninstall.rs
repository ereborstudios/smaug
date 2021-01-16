use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;

#[derive(Debug)]
pub struct Uninstall;

impl Command for Uninstall {
    fn run(&self, _matches: &ArgMatches) -> CommandResult {
        // let path = Path::new(matches.value_of("FILE").expect("No matches found"));

        Ok(Box::new("Uninstalled DragonRuby".to_string()))
    }
}

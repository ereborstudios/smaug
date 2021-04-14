use clap::ArgMatches;
use serde::Serialize;
use std::fmt::Display;

pub type CommandResult = Result<Box<dyn Json>, Box<dyn Json>>;

pub trait Json {
    fn to_json(&self) -> String;
    fn to_string(&self) -> String;
}

impl<T: Serialize + Display> Json for T {
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Could not convert to json")
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

pub trait Command {
    fn run(&self, matches: &ArgMatches) -> CommandResult;
}

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use serde::Serialize;
use smaug::dragonruby::Version;

#[derive(Debug)]
pub struct List;

#[derive(Debug, Serialize, Display)]
#[display(fmt = "{}", "format_versions(versions)")]
pub struct ListResult {
    versions: Vec<Version>,
}

impl Command for List {
    fn run(&self, _matches: &ArgMatches) -> CommandResult {
        match smaug::dragonruby::list_installed() {
            Ok(versions) => Ok(Box::new(ListResult {
                versions: versions.iter().map(|dr| dr.clone().version).collect(),
            })),
            Err(_) => Ok(Box::new(ListResult { versions: vec![] })),
        }
    }
}

fn format_versions(versions: &[Version]) -> String {
    if versions.is_empty() {
        "No DragonRuby versions installed.".to_string()
    } else {
        versions
            .iter()
            .map(|version| format!("* {}\n", version))
            .collect::<Vec<String>>()
            .join("")
    }
}

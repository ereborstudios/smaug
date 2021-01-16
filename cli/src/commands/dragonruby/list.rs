use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;

#[derive(Debug)]
pub struct List;

impl Command for List {
    fn run(&self, _matches: &ArgMatches) -> CommandResult {
        match smaug::dragonruby::list_installed() {
            Ok(versions) => {
                if versions.is_empty() {
                    Ok(Box::new("No DragonRuby versions installed."))
                } else {
                    let message = versions
                        .iter()
                        .map(|version| {
                            format!(
                                "* {}\n",
                                version.path.file_name().unwrap().to_str().unwrap()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("");

                    Ok(Box::new(message))
                }
            }
            Err(..) => Ok(Box::new("No DragonRuby versions installed.")),
        }
    }
}

use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use log::*;
use std::path::Path;

#[derive(Debug)]
pub struct New;

impl Command for New {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("New Command");

        let latest = smaug::dragonruby::latest();
        if let Err(e) = latest {
            return Err(Box::new(e));
        }
        let latest = latest.unwrap();

        debug!("Latest DragonRuby: {}", latest);

        let directory: &str = matches.value_of("PATH").unwrap();
        debug!("Directory: {}", directory);
        let path = Path::new(directory);

        let source = latest.install_dir().join("mygame");
        smaug::util::dir::copy_directory(&source, &path.to_path_buf())
            .expect("Installed DragonRuby doesn't have mygame directory.");

        let gitignore = include_str!("../../templates/gitignore");
        let gitignore_path = path.join(".gitignore");

        std::fs::write(gitignore_path, gitignore).expect("Couldn't write .gitignore.");

        crate::commands::init::Init.run(matches)?;

        Ok(Box::new("Created your new DragonRuby project."))
    }
}

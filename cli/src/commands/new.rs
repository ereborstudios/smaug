use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct New;

#[derive(Debug, Display, Error, Serialize)]
enum Error {
    #[display(fmt = "DragonRuby is not installed. See smaug dragonruby help install for details.")]
    DragonRubyNotFound,
    #[display(fmt = "Project initialization failed")]
    InitFailed,
}

#[derive(Debug, Serialize, Display)]
#[display(fmt = "Created your new DragonRuby project.")]
pub struct NewResult {
    path: PathBuf,
}

impl Command for New {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("New Command");

        let latest = smaug_lib::dragonruby::latest();
        if let Err(..) = latest {
            return Err(Box::new(Error::DragonRubyNotFound {}));
        }
        let latest = latest.unwrap();

        debug!("Latest DragonRuby: {}", latest);

        let directory: &str = matches.value_of("PATH").unwrap();
        debug!("Directory: {}", directory);
        let path = Path::new(directory);

        let source = latest.install_dir().join("mygame");
        smaug_lib::util::dir::copy_directory(&source, path.to_path_buf())
            .expect("Installed DragonRuby doesn't have mygame directory.");

        let gitignore = include_str!("../../templates/gitignore.template");
        let gitignore_path = path.join(".gitignore");

        std::fs::write(gitignore_path, gitignore).expect("Couldn't write .gitignore.");

        let smaugignore = include_str!("../../templates/smaugignore.template");
        let smaugignore_path = path.join(".smaugignore");

        std::fs::write(smaugignore_path, smaugignore).expect("Couldn't write .smaugignore.");

        if crate::commands::init::Init.run(matches).is_err() {
            return Err(Box::new(Error::InitFailed));
        }

        Ok(Box::new(NewResult {
            path: path.to_path_buf(),
        }))
    }
}

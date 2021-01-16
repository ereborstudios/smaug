use crate::command::Command;
use crate::command::CommandResult;
use crate::util::dir::copy_directory;
use clap::ArgMatches;
use derive_more::Display;
use derive_more::Error;
use log::*;
use smaug::dragonruby;
use smaug::dragonruby::DragonRuby;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Install;

#[derive(Debug, Display, Error)]

enum Error {
    #[display(fmt = "Could not find DragonRuby at {}", "path.display()")]
    DragonRubyNotFound { path: PathBuf },
}

impl Command for Install {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Install Command");
        let path = Path::new(matches.value_of("FILE").expect("No Matches Found"));
        let dr = dragonruby::new(&path).expect("Couldn't find DragonRuby");

        match install(&dr) {
            Ok(installed) => Ok(Box::new(format!("Installed {}", installed.version))),
            Err(..) => Err(Box::new(Error::DragonRubyNotFound {
                path: path.to_path_buf(),
            })),
        }
    }
}

fn install(dr: &DragonRuby) -> io::Result<DragonRuby> {
    let source = dr.path.clone();
    let destination = dr.install_dir();

    copy_directory(&source, &destination)?;

    let dr = dragonruby::new(&destination).expect("Could not find DragonRuby");

    Ok(dr)
}

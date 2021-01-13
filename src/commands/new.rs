use crate::commands::init;
use crate::dragonruby;
use crate::smaug;
use crate::utils::copy_directory;
use log::*;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

pub fn call(matches: &clap::ArgMatches) -> io::Result<()> {
    dragonruby::ensure_installed();

    let path = matches.value_of("PATH").unwrap();
    let destination = Path::new(path);
    debug!("Project path: {}", destination.to_str().unwrap());

    if destination.exists() {
        smaug::print_error(format!("{} already exists", path));
        process::exit(exitcode::DATAERR);
    }

    trace!("Creating directory {}", destination.to_str().unwrap());
    fs::create_dir(destination)?;

    let template = dragonruby::dragonruby_directory().join("mygame");
    debug!("Template Directory: {}", template.to_str().unwrap());
    copy_directory(&template, &destination)?;

    init::generate_config(&destination.join("Smaug.toml").as_path());

    info!("Created Smaug.toml edit the values with your project's information.");
    scrawl::editor::new()
        .file(&destination.join("Smaug.toml").as_path())
        .edit()
        .open()
        .unwrap();

    init::generate_gitignore(&destination);

    Ok(())
}

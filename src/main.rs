extern crate exitcode;

mod commands;
mod dependency;
mod dragonruby;
mod file;
mod git;
mod lock;
mod project_config;
mod smaug;
mod url;
mod utils;

use clap::clap_app;
use std::io;

fn main() -> io::Result<()> {
    let matches = clap_app!(smaug =>
        (version: "0.1.0")
        (author: "Matt Pruitt <matt@guitsaru.com>")
        (about: "Installs DragonRuby dependencies")
        (setting: clap::AppSettings::ArgRequiredElseHelp)

        (@arg verbose: -v... --verbose... +global takes_value(false) "Displays more information")
        (@arg quiet: -q --quiet +global takes_value(false) "Silence all output")

        (@subcommand dragonruby =>
            (about: "Manages your local DragonRuby installation.")
            (setting: clap::AppSettings::ArgRequiredElseHelp)
            (@subcommand install =>
                (about: "Installs DragonRuby.")
                (@arg FILE: +required "The location of the DragonRuby Game Toolkit zip file.")
            )
            (@subcommand uninstall =>
                (about: "Uninstalls DragonRuby.")
            )
        )
        (@subcommand new =>
            (about: "Start a new DragonRuby project")
            (@arg PATH: +required "The path to your new project")
        )
        (@subcommand init =>
            (about: "Initializes an existing project as a Smaug project.")
            (@arg PATH: "The path to your project. Defaults to the current directory.")
        )
        (@subcommand package =>
            (about: "Initializes an existing library as a Smaug package.")
            (@arg PATH: "The path to your package. Defaults to the current directory.")
        )
        (@subcommand run =>
            (about: "Runs your DragonRuby project.")
            (@arg PATH: "The path to your project. Defaults to the current directory.")
        )
        (@subcommand build =>
            (about: "Builds your DragonRuby project.")
            (@arg PATH: "The path to your project. Defaults to the current directory.")
        )
        (@subcommand publish =>
            (about: "Publish your DragonRuby project to Itch.io")
            (@arg PATH: "The path to your project. Defaults to the current directory.")
        )
        (@subcommand install =>
            (about: "Installs dependencies from Smaug.toml.")
            (@arg PATH: "The path to your project. Defaults to the current directory.")
            (@arg package: --package takes_value(true) "The location of a package to install. If not provided, will install from Smaug.toml.")
        )
    )
    .get_matches();
    start_log(&matches);

    match matches.subcommand_name() {
        Some("dragonruby") => {
            let matches = matches.subcommand_matches("dragonruby").unwrap();
            match matches.subcommand_name() {
                Some("install") => commands::dragonruby::install::call(
                    matches.subcommand_matches("install").unwrap(),
                ),
                Some("uninstall") => commands::dragonruby::uninstall::call(
                    matches.subcommand_matches("uninstall").unwrap(),
                ),
                _ => unreachable!(),
            }
        }
        Some("new") => commands::new::call(matches.subcommand_matches("new").unwrap())?,
        Some("run") => commands::run::call(matches.subcommand_matches("run").unwrap()),
        Some("build") => commands::build::call(matches.subcommand_matches("build").unwrap())?,
        Some("publish") => commands::publish::call(matches.subcommand_matches("publish").unwrap())?,
        Some("init") => commands::init::call(matches.subcommand_matches("init").unwrap()),
        Some("package") => commands::package::call(matches.subcommand_matches("package").unwrap()),
        Some("install") => commands::install::call(matches.subcommand_matches("install").unwrap())?,
        _ => unreachable!(),
    }

    Ok(())
}

fn start_log(matches: &clap::ArgMatches) {
    let quiet = matches.is_present("quiet");
    let verbosity = matches.occurrences_of("verbose") as usize;

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbosity + 2)
        .timestamp(stderrlog::Timestamp::Off)
        .show_level(false)
        .init()
        .unwrap();
}

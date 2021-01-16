extern crate derive_more;

mod command;
mod commands;
mod util;

use crate::command::Command;
use crate::commands::package::Package;
use clap::clap_app;
use commands::{dragonruby::DragonRuby, init::Init};
use log::*;

fn main() {
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
                (@arg VERSION: +required "The version of DragonRuby to uninstall.")
            )
            (@subcommand list =>
                (about: "Lists installed DragonRuby versions.")
            )
        )
        (@subcommand package =>
            (about: "Manages your DragonRuby package.")
            (setting: clap::AppSettings::ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Initializes an existing package as a Smaug project.")
                (@arg PATH: "The path to your package. Defaults to the current directory.")
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
        )
    )
    .get_matches();

    start_log(&matches);

    let command: Box<dyn Command> = match matches.subcommand_name() {
        Some("dragonruby") => Box::new(DragonRuby),
        Some("init") => Box::new(Init),
        Some("package") => Box::new(Package),
        _ => unreachable!(),
    };

    let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

    let result = command.run(&subcommand_matches.expect("No subcommand matches"));

    info!("");
    match result {
        Ok(message) => info!("{}", message),
        Err(message) => error!("{}", message),
    }

    print_message()
}

fn print_message() {
    info!("");
    info!("Thanks for using Smaug!");
    info!("🦗 Find a bug? File an issue: https://github.com/guitsaru/smaug/issues");
    info!("🙋 Have a question? Start a discussion: https://github.com/guitsaru/smaug/discussions");
    info!("💬 Want to chat? Join us on Discord: https://discord.gg/3MEsGjxZ");
    info!("");
}

fn start_log(matches: &clap::ArgMatches) {
    let quiet = matches.is_present("quiet");
    let verbosity = matches.occurrences_of("verbose") as usize;

    stderrlog::new()
        .module(module_path!())
        .module("smaug")
        .quiet(quiet)
        .verbosity(verbosity + 2)
        .timestamp(stderrlog::Timestamp::Off)
        .show_level(false)
        .init()
        .unwrap();
}

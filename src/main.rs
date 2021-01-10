extern crate exitcode;
mod commands;
mod dragonruby;
mod project_config;
mod smaug;
use clap::clap_app;

fn main() {
    let matches = clap_app!(smaug =>
        (version: "0.1.0")
        (author: "Matt Pruitt <matt@guitsaru.com>")
        (about: "Installs DragonRuby dependencies")
        (setting: clap::AppSettings::ArgRequiredElseHelp)
        (@arg verbose: -v --verbose "Displays more information")
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
        (@subcommand run =>
            (about: "Runs a DragonRuby project")
            (@arg PATH: "The path to your new project. Defaults to the current directory.")
        )
        (@subcommand init =>
            (about: "Initializes an existing project as a Smaug project.")
            (@arg PATH: "The path to your new project. Defaults to the current directory.")
        )
    )
    .get_matches();

    match matches.subcommand_name() {
        Some("dragonruby") => {
            let matches = matches.subcommand_matches("dragonruby").unwrap();
            match matches.subcommand_name() {
                Some("install") => commands::dragonruby::install::call(
                    &matches.subcommand_matches("install").unwrap(),
                ),
                Some("uninstall") => commands::dragonruby::uninstall::call(
                    &matches.subcommand_matches("uninstall").unwrap(),
                ),
                _ => unreachable!(),
            }
        }
        Some("new") => commands::new::call(&matches.subcommand_matches("new").unwrap()),
        Some("run") => commands::run::call(&matches.subcommand_matches("run").unwrap()),
        Some("init") => commands::init::call(&matches.subcommand_matches("init").unwrap()),
        _ => unreachable!(),
    }
}

extern crate exitcode;
mod dragonruby;
mod install;
mod new;
use clap::clap_app;

fn main() {
    let matches = clap_app!(smaug =>
        (version: "0.1.0")
        (author: "Matt Pruitt <matt@guitsaru.com>")
        (about: "Installs DragonRuby dependencies")
        (setting: clap::AppSettings::ArgRequiredElseHelp)
        (@arg verbose: -v --verbose "Displays more information")
        (@subcommand install =>
            (about: "Installs DragonRuby")
            (@arg FILE: +required "The location of the DragonRuby Game Toolkit zip file")
        )
        (@subcommand new =>
            (about: "Start a new DragonRuby project")
            (@arg PATH: +required "The path to your new project")
        )
    )
    .get_matches();

    match matches.subcommand_name() {
        Some("install") => install::install(&matches.subcommand_matches("install").unwrap()),
        Some("new") => new::new(&matches.subcommand_matches("new").unwrap()),
        _ => unreachable!(),
    }
}

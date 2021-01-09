extern crate exitcode;
mod dragonruby;
mod install;
use clap::clap_app;

fn main() {
    let matches = clap_app!(smaug =>
        (version: "0.1.0")
        (author: "Matt Pruitt <matt@guitsaru.com>")
        (about: "Installs DragonRuby dependencies")
        (@arg verbose: -v --verbose "Displays more information")
        (@subcommand install =>
            (about: "Installs DragonRuby")
            (@arg FILE: +required "The location of the DragonRuby Game Toolkit zip file")
        )
    )
    .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("install") {
        install::install(matches);
    }
}

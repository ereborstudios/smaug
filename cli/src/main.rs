extern crate derive_more;

mod command;
mod commands;
mod game_metadata;

use crate::command::Command;
use crate::commands::bind::Bind;
use crate::commands::package::Package;
use crate::commands::run::Run;
use clap::clap_app;
use commands::install::Install;
use commands::{
    add::Add, build::Build, config::Config, docs::Docs, dragonruby::DragonRuby, init::Init,
    new::New, publish::Publish,
};
use log::*;

fn main() {
    let matches = clap_app!(smaug =>
        (version: "0.5.0")
        (author: "Matt Pruitt <matt@guitsaru.com>")
        (about: "Create games and share packages with the DragonRuby community")
        (setting: clap::AppSettings::SubcommandRequiredElseHelp)

        (@arg verbose: -v... --verbose... +global takes_value(false) "Displays more information")
        (@arg json: --json +global takes_value(false) "Returns JSON")
        (@arg quiet: -q --quiet +global takes_value(false) "Silence all output")

        (@subcommand dragonruby =>
            (about: "Manages your local DragonRuby installation.")
            (setting: clap::AppSettings::SubcommandRequiredElseHelp)
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
            (setting: clap::AppSettings::SubcommandRequiredElseHelp)
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
        (@subcommand run =>
            (about: "Runs your DragonRuby project.")
            (setting: clap::AppSettings::TrailingVarArg)
            (setting: clap::AppSettings::AllowLeadingHyphen)
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg http: --http "Run your HTML5 game")
            (@arg DRAGONRUBY_ARGS: ... "dragonruby command options")
        )
        (@subcommand build =>
            (about: "Builds your DragonRuby project.")
            (setting: clap::AppSettings::TrailingVarArg)
            (setting: clap::AppSettings::AllowLeadingHyphen)
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg DRAGONRUBY_ARGS: ... "dragonruby command options")
        )
        (@subcommand publish =>
            (about: "Publish your DragonRuby project to Itch.io")
            (setting: clap::AppSettings::TrailingVarArg)
            (setting: clap::AppSettings::AllowLeadingHyphen)
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg DRAGONRUBY_ARGS: ... "dragonruby-publish command options")
        )
        (@subcommand bind =>
            (about: "Create bindings for c extensions (Pro only)")
            (setting: clap::AppSettings::TrailingVarArg)
            (setting: clap::AppSettings::AllowLeadingHyphen)
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg output: --output -o +required +takes_value "The location of the generated bindings.")
            (@arg FILE: +required "The file to generate bindings for.")
            (@arg DRAGONRUBY_ARGS: ... "dragonruby-publish command options")
        )
        (@subcommand add =>
            (about: "Adds a dependency to the project.")
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg PACKAGE: +required "The package to add to your project's dependencies")
        )
        (@subcommand install =>
            (about: "Installs dependencies from Smaug.toml.")
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
        )
        (@subcommand add =>
            (about: "Add a dependency to Smaug.toml")
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
            (@arg PACKAGE: +required "The location of a package to add")
        )
        (@subcommand config =>
            (about: "Displays your current project's Smaug configuration")
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
        )

        (@subcommand docs =>
            (about: "Opens DragonRuby docs in your web browser")
            (@arg path: --path -p +takes_value "The path to your project. Defaults to the current directory.")
        )
    )
    .get_matches();

    let command: Option<Box<dyn Command>> = match matches.subcommand_name() {
        Some("build") => Some(Box::new(Build)),
        Some("dragonruby") => Some(Box::new(DragonRuby)),
        Some("init") => Some(Box::new(Init)),
        Some("install") => Some(Box::new(Install)),
        Some("new") => Some(Box::new(New)),
        Some("package") => Some(Box::new(Package)),
        Some("publish") => Some(Box::new(Publish)),
        Some("run") => Some(Box::new(Run)),
        Some("add") => Some(Box::new(Add)),
        Some("bind") => Some(Box::new(Bind)),
        Some("config") => Some(Box::new(Config)),
        Some("docs") => Some(Box::new(Docs)),
        _ => None,
    };

    if let Some(cmd) = command {
        start_log(&matches);

        let subcommand_matches = matches.subcommand_matches(matches.subcommand_name().unwrap());

        let json = matches.is_present("json");
        let result = cmd.run(subcommand_matches.expect("No subcommand matches"));

        info!("");
        match result {
            Ok(message) => {
                if json {
                    println!("{}", message.to_json())
                } else {
                    println!("{}", message.to_string())
                }
            }
            Err(message) => {
                if json {
                    println!("{{\"error\": {}}}", message.to_json())
                } else {
                    error!("{}", message.to_string())
                }
            }
        }
        print_message()
    }
}

fn print_message() {
    info!("");
    info!("Thanks for using Smaug!");
    info!("📦 Explore the package registry at https://smaug.dev/");
    info!("🦗 Find a bug? File an issue: https://github.com/ereborstudios/smaug/issues");
    info!("🙋 Have a question? Start a discussion: https://github.com/ereborstudios/smaug/discussions");
    info!("💬 Want to chat? Join us on Discord: https://discord.gg/rwT64EtDee");
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

use directories::ProjectDirs;
use log::*;
use std::path::PathBuf;
use std::process;

pub fn print_error<S: Into<String>>(message: S) {
    info!("");
    error!("{}", message.into());
    info!("");
    info!("Thanks for using Smaug!");
    info!("🦗 Find a bug? File an issue: https://github.com/guitsaru/smaug/issues");
    info!("🙋 Have a question? Start a discussion: https://github.com/guitsaru/smaug/discussions");
    info!("💬 Want to chat? Join us on Discord: https://discord.gg/3MEsGjxZ");
    info!("");
}

pub fn data_dir() -> PathBuf {
    return project_dirs().data_dir().to_path_buf();
}

pub fn cache_dir() -> PathBuf {
    return project_dirs().cache_dir().to_path_buf();
}

fn project_dirs() -> ProjectDirs {
    let project_dirs = ProjectDirs::from("org", "Erebor Studios", "Smaug");

    match project_dirs {
        Some(dirs) => return dirs,
        None => {
            print_error("No project directories found");
            process::exit(exitcode::OSFILE);
        }
    }
}

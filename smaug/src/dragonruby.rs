use crate::{config::Config, smaug};
use derive_more::Display;
use derive_more::Error;
use log::*;
use semver::Version as SemVer;
use semver::VersionReq;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Display)]
pub enum Edition {
    #[display(fmt = "")]
    Standard,
    #[display(fmt = "Pro")]
    Pro,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Display)]
#[display(
    fmt = "DragonRuby {} {}.{}",
    "edition",
    "version.major",
    "version.minor"
)]
pub struct Version {
    pub edition: Edition,
    pub version: SemVer,
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}", "version")]
pub struct DragonRuby {
    pub path: PathBuf,
    pub version: Version,
}

#[derive(Debug, Error, Display)]
pub enum DragonRubyError {
    #[display(fmt = "Could not find a valid DragonRuby at {}", "path.display()")]
    DragonRubyNotFound { path: PathBuf },
    #[display(
        fmt = "There is no version of DragonRuby installed.\nInstall with `smaug dragonruby install`."
    )]
    DragonRubyNotInstalled,
}

type DragonRubyResult = Result<DragonRuby, DragonRubyError>;

pub fn new<P: AsRef<Path>>(path: &P) -> DragonRubyResult {
    let dragonruby_path = path.as_ref();

    if dragonruby_path.is_dir() {
        parse_dragonruby_dir(&dragonruby_path)
    } else if zip_extensions::is_zip(&dragonruby_path.to_path_buf()) {
        parse_dragonruby_zip(&dragonruby_path)
    } else {
        Err(DragonRubyError::DragonRubyNotFound {
            path: dragonruby_path.to_path_buf(),
        })
    }
}

impl DragonRuby {
    pub fn install_dir(&self) -> PathBuf {
        let location = smaug::data_dir().join("dragonruby");
        match self.version.edition {
            Edition::Pro => location.join(format!(
                "pro-{}.{}",
                self.version.version.major, self.version.version.minor
            )),
            Edition::Standard => location.join(format!(
                "{}.{}",
                self.version.version.major, self.version.version.minor
            )),
        }
    }
}

pub fn latest() -> DragonRubyResult {
    let list = list_installed();

    match list {
        Err(..) => Err(DragonRubyError::DragonRubyNotInstalled),
        Ok(mut versions) => {
            if versions.is_empty() {
                Err(DragonRubyError::DragonRubyNotInstalled)
            } else {
                versions.sort_by(|a, b| a.version.partial_cmp(&b.version).unwrap());
                let latest = versions.last().unwrap();

                Ok((*latest).clone())
            }
        }
    }
}

pub fn configured_version(config: &Config) -> Option<DragonRuby> {
    let version = VersionReq::parse(config.dragonruby.version.as_str())
        .expect("Not a valid DragonRuby version.");
    let edition = if config.dragonruby.edition == "pro" {
        Edition::Pro
    } else {
        Edition::Standard
    };

    let mut installed = list_installed().expect("Could not list installed.");
    installed.sort_by(|a, b| a.version.partial_cmp(&b.version).unwrap());
    let matched = installed
        .iter()
        .find(|v| version.matches(&v.version.version) && v.version.edition >= edition);

    match matched {
        Some(dragonruby) => Some(dragonruby.clone()),
        None => None,
    }
}

pub fn list_installed() -> io::Result<Vec<DragonRuby>> {
    let location = smaug::data_dir().join("dragonruby");
    fs::create_dir_all(location.as_path())?;

    let folders = fs::read_dir(location).expect("DragonRuby install folder not found.");
    let versions: Vec<DragonRuby> = folders
        .map(|folder| {
            let path = folder.expect("Invalid folder");
            parse_dragonruby_dir(&path.path())
        })
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .collect();

    Ok(versions)
}

pub fn dragonruby_bin_name() -> String {
    if cfg!(windows) {
        return "dragonruby.exe".to_string();
    } else {
        return "dragonruby".to_string();
    }
}

pub fn dragonruby_bind_name() -> String {
    if cfg!(windows) {
        return "dragonruby-bind.exe".to_string();
    } else {
        return "dragonruby-bind".to_string();
    }
}

pub fn dragonruby_httpd_name() -> String {
    if cfg!(windows) {
        return "dragonruby-httpd.exe".to_string();
    } else {
        return "dragonruby-httpd".to_string();
    }
}

pub fn dragonruby_publish_name() -> String {
    if cfg!(windows) {
        return "dragonruby-publish.exe".to_string();
    } else {
        return "dragonruby-publish".to_string();
    }
}

fn parse_dragonruby_zip(path: &Path) -> DragonRubyResult {
    let cache = smaug::cache_dir();
    trace!("Unzipping DragonRuby from {}", path.display());
    if cache.is_dir() {
        fs::remove_dir_all(cache.clone()).expect("Couldn't clear cache");
    }
    zip_extensions::zip_extract(&path.to_path_buf(), &cache).expect("Could not extract zip");
    trace!("Unzipped DragonRuby to {}", cache.display());
    let mut dir = fs::read_dir(cache.as_path()).expect("Could not read from cache");
    let unzipped_to = dir
        .next()
        .expect("DragonRuby zip had no files.")
        .expect("DragonRuby zip had no files");

    parse_dragonruby_dir(&unzipped_to.path())
}

fn parse_dragonruby_dir(path: &Path) -> DragonRubyResult {
    trace!("Parsing DragonRuby directory at {}", path.display());
    let edition: Edition;

    if !path.is_dir() {
        return Err(DragonRubyError::DragonRubyNotFound {
            path: path.to_path_buf(),
        });
    };

    let dragonruby_bin = path.join(dragonruby_bin_name());
    debug!("DragonRuby bin {}", dragonruby_bin.display());
    let dragonruby_bind_bin = path.join(dragonruby_bind_name());
    debug!("DragonRuby Bind bin {}", dragonruby_bind_bin.display());
    let changelog = path.join("CHANGELOG.txt");
    debug!("Changelog {}", changelog.display());

    if !dragonruby_bin.exists() || !changelog.exists() {
        return Err(DragonRubyError::DragonRubyNotFound {
            path: path.to_path_buf(),
        });
    };

    let changelog_contents =
        fs::read_to_string(changelog).expect("CHANGELOG.txt could not be read.");

    let first_line = changelog_contents
        .lines()
        .next()
        .expect("No lines in changelog");

    debug!("First Line: {}", first_line);

    let latest = first_line.replace("* ", "");

    debug!("Latest: {}", latest);

    let version =
        SemVer::parse(format!("{}.0", latest.as_str()).as_str()).expect("not a valid version");
    debug!("Version: {}", version);

    if dragonruby_bind_bin.exists() {
        edition = Edition::Pro;
    } else {
        edition = Edition::Standard;
    }

    let dragonruby = DragonRuby {
        path: path.to_path_buf(),
        version: Version { edition, version },
    };

    Ok(dragonruby)
}

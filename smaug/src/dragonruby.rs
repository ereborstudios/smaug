use crate::smaug;
use derive_more::Display;
use derive_more::Error;
use log::*;
use semver::Version as SemVer;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Edition {
    Standard,
    Pro,
}

#[derive(Debug)]
pub struct Version {
    pub edition: Edition,
    pub version: SemVer,
}

#[derive(Debug)]
pub struct DragonRuby {
    pub path: PathBuf,
    pub version: Version,
}

#[derive(Debug, Error, Display)]
pub enum DragonRubyError {
    #[display(fmt = "Could not find a valid DragonRuby at {}", "path.display()")]
    DragonRubyNotFound { path: PathBuf },
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

    let dragonruby_bin = path.join("dragonruby");
    debug!("DragonRuby bin {}", dragonruby_bin.display());
    let dragonruby_bind_bin = path.join("dragonruby-bind");
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

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.edition {
            Edition::Pro => write!(
                fmt,
                "DragonRuby Pro {}.{}",
                self.version.major, self.version.minor
            )?,
            Edition::Standard => write!(
                fmt,
                "DragonRuby {}.{}",
                self.version.major, self.version.minor
            )?,
        }

        Ok(())
    }
}

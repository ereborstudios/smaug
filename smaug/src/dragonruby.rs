use crate::smaug;
use log::*;
use semver::Version as SemVer;
use std::fmt;
use std::fs;
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

#[derive(Debug)]
pub enum DragonRubyError {
    DragonRubyNotFound(PathBuf),
}

type DragonRubyResult = Result<DragonRuby, DragonRubyError>;

pub fn new<P: AsRef<Path>>(path: &P) -> DragonRubyResult {
    let dragonruby_path = path.as_ref();

    if dragonruby_path.is_dir() {
        parse_dragonruby_dir(&dragonruby_path)
    } else if zip_extensions::is_zip(&dragonruby_path.to_path_buf()) {
        parse_dragonruby_zip(&dragonruby_path)
    } else {
        Err(DragonRubyError::DragonRubyNotFound(
            dragonruby_path.to_path_buf(),
        ))
    }
}

impl DragonRuby {
    pub fn install_dir(&self) -> PathBuf {
        let location = smaug::data_dir().join("dragonruby");
        match self.version.edition {
            Edition::Pro => location.join(format!("{}-pro", self.version.version)),
            Edition::Standard => location.join(format!("{}", self.version.version)),
        }
    }
}

fn parse_dragonruby_zip(path: &Path) -> DragonRubyResult {
    let cache = smaug::cache_dir();
    zip_extensions::zip_extract(&path.to_path_buf(), &cache).expect("Could not extract zip");
    let mut dir = fs::read_dir(cache.as_path()).expect("Could not read from cache");
    let unzipped_to = dir
        .next()
        .expect("DragonRuby zip had no files.")
        .expect("DragonRuby zip had no files");

    parse_dragonruby_dir(&unzipped_to.path())
}

fn parse_dragonruby_dir(path: &Path) -> DragonRubyResult {
    let edition: Edition;

    let dragonruby_bin = path.join("dragonruby");
    debug!("DragonRuby bin {}", dragonruby_bin.display());
    let dragonruby_bind_bin = path.join("dragonruby-bind");
    debug!("DragonRuby Bind bin {}", dragonruby_bind_bin.display());
    let changelog = path.join("CHANGELOG.txt");
    debug!("Changelog {}", changelog.display());

    if !dragonruby_bin.exists() || !changelog.exists() {
        return Err(DragonRubyError::DragonRubyNotFound(path.to_path_buf()));
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

use crate::{config::Config, smaug};
use derive_more::Display;
use derive_more::Error;
use log::*;
use semver::Version as SemVer;
use semver::VersionReq;
use serde::Serialize;
use serde::Serializer;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Display)]
pub enum Edition {
    #[display(fmt = "")]
    Standard,
    #[display(fmt = "Indie")]
    Indie,
    #[display(fmt = "Pro")]
    Pro,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Display)]
#[display(
    fmt = "DragonRuby {} {}.{} ({})",
    "edition",
    "version.major",
    "version.minor",
    "identifier"
)]
pub struct Version {
    pub edition: Edition,
    pub version: SemVer,
    pub identifier: String,
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
        parse_dragonruby_dir(dragonruby_path)
    } else if zip_extensions::is_zip(&dragonruby_path.to_path_buf()) {
        parse_dragonruby_zip(dragonruby_path)
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
            Edition::Indie => location.join(format!(
                "indie-{}.{}",
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
    } else if config.dragonruby.edition == "indie" {
        Edition::Indie
    } else {
        Edition::Standard
    };

    let mut installed = list_installed().expect("Could not list installed.");
    installed.sort_by(|a, b| a.version.partial_cmp(&b.version).unwrap());
    let matched = installed
        .iter()
        .find(|v| version.matches(&v.version.version) && v.version.edition >= edition);

    matched.map(|dragonruby| dragonruby.to_owned())
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

pub fn dragonruby_docs_path() -> String {
    "docs/docs.html".to_string()
}

pub fn dragonruby_bin_name() -> String {
    if cfg!(windows) {
        "dragonruby.exe".to_string()
    } else {
        "dragonruby".to_string()
    }
}

pub fn dragonruby_bind_name() -> String {
    if cfg!(windows) {
        "dragonruby-bind.exe".to_string()
    } else {
        "dragonruby-bind".to_string()
    }
}

pub fn dragonruby_httpd_name() -> String {
    if cfg!(windows) {
        "dragonruby-httpd.exe".to_string()
    } else {
        "dragonruby-httpd".to_string()
    }
}

pub fn dragonruby_publish_name() -> String {
    if cfg!(windows) {
        "dragonruby-publish.exe".to_string()
    } else {
        "dragonruby-publish".to_string()
    }
}

fn parse_dragonruby_zip(path: &Path) -> DragonRubyResult {
    let cache = smaug::cache_dir();
    trace!("Unzipping DragonRuby from {}", path.display());
    rm_rf::ensure_removed(cache.clone()).expect("Couldn't clear cache");
    zip_extensions::zip_extract(&path.to_path_buf(), &cache).expect("Could not extract zip");
    trace!("Unzipped DragonRuby to {}", cache.display());

    parse_dragonruby_dir(&cache)
}

fn find_base_dir(path: &Path) -> io::Result<PathBuf> {
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "did not pass in a directory",
        ));
    }

    let files = path.read_dir()?;

    for entry in files {
        let entry = entry?.path();
        trace!("Looking for dragonruby at {:?}", entry);

        if entry.is_dir() {
            let bd = find_base_dir(entry.as_path());

            if bd.is_ok() {
                return bd;
            }
        } else if entry
            .file_name()
            .expect("entry did not have a file name")
            .to_string_lossy()
            == dragonruby_bin_name()
        {
            let parent = entry.parent();

            match parent {
                Some(parent_path) => return Ok(parent_path.to_path_buf()),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "could not find DragonRuby directory",
                    ))
                }
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "could not find DragonRuby directory",
    ))
}

fn parse_dragonruby_dir(path: &Path) -> DragonRubyResult {
    trace!("Parsing DragonRuby directory at {}", path.display());
    let edition: Edition;

    if !path.is_dir() {
        trace!("{:?} is not a directory", path);
        return Err(DragonRubyError::DragonRubyNotFound {
            path: path.to_path_buf(),
        });
    };

    let base_path = match find_base_dir(path) {
        Ok(base) => base,
        Err(_) => {
            trace!("No base path found");
            return Err(DragonRubyError::DragonRubyNotFound {
                path: path.to_path_buf(),
            });
        }
    };

    let dragonruby_bin = base_path.join(dragonruby_bin_name());
    debug!("DragonRuby bin {}", dragonruby_bin.display());
    let dragonruby_bind_bin = base_path.join(dragonruby_bind_name());
    debug!("DragonRuby Bind bin {}", dragonruby_bind_bin.display());
    let dragonruby_android_stub = base_path.join(".dragonruby/stubs/android");
    debug!("DragonRuby iOS app bin {}", dragonruby_bind_bin.display());
    let mut changelog = base_path.join("CHANGELOG.txt");
    if !changelog.exists() {
        changelog = base_path.join("CHANGELOG-CURR.txt");
    }
    debug!("Changelog {}", changelog.display());

    if !dragonruby_bin.exists() || !changelog.exists() {
        return Err(DragonRubyError::DragonRubyNotFound { path: base_path });
    };

    let changelog_contents = fs::read_to_string(changelog).expect("CHANGELOG could not be read.");

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

    if dragonruby_android_stub.exists() {
        edition = Edition::Pro;
    } else if dragonruby_bind_bin.exists() {
        edition = Edition::Indie;
    } else {
        edition = Edition::Standard;
    }

    let dragonruby = DragonRuby {
        path: base_path.clone(),
        version: Version {
            edition,
            version,
            identifier: base_path.file_name().unwrap().to_string_lossy().to_string(),
        },
    };

    Ok(dragonruby)
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

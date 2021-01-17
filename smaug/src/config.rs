use derive_more::Display;
use derive_more::Error;
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: Option<Project>,
    pub dragonruby: DragonRuby,
    pub itch: Option<Itch>,
    pub dependencies: Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub title: String,
    pub version: String,
    pub authors: Vec<String>,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct DragonRuby {
    pub version: String,
    pub edition: String,
}

#[derive(Debug, Deserialize)]
pub struct Itch {
    pub url: String,
    pub username: String,
}

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "Could not find Smaug.toml at {}", "path.display()")]
    FileNotFound { path: PathBuf },
    #[display(
        fmt = "Could not parse Smaug.toml at {}: {}",
        "path.display()",
        "parent"
    )]
    ParseError {
        path: PathBuf,
        parent: toml::de::Error,
    },
}

pub fn load<P: AsRef<Path>>(path: &P) -> Result<Config, Error> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(Error::FileNotFound {
            path: path.to_path_buf(),
        });
    }

    let contents = std::fs::read_to_string(path).expect("Could not read Smaug.toml");
    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::ParseError {
            path: path.to_path_buf(),
            parent: err,
        }),
    }
}

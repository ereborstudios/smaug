use derive_more::Display;
use derive_more::Error;
use semver::VersionReq;
use serde::de;
use serde::de::Deserializer;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: Option<Project>,
    pub dragonruby: DragonRuby,
    pub itch: Option<Itch>,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencyOptions>,
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

#[derive(Debug)]
pub enum DependencyOptions {
    Dir {
        dir: PathBuf,
    },
    File {
        file: PathBuf,
    },
    Git {
        branch: Option<String>,
        repo: String,
    },
    Registry {
        version: VersionReq,
    },
    Url {
        url: String,
    },
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
impl<'de> Deserialize<'de> for DependencyOptions {
    fn deserialize<D>(deserializer: D) -> Result<DependencyOptions, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DependencyOptionsVisitor;

        impl<'de> Visitor<'de> for DependencyOptionsVisitor {
            type Value = DependencyOptions;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DependencyOptions")
            }

            fn visit_str<E>(self, value: &str) -> Result<DependencyOptions, E>
            where
                E: de::Error,
            {
                let path = Path::new(value);
                if let Ok(version) = VersionReq::parse(value) {
                    Ok(DependencyOptions::Registry { version })
                } else if let Some("git") = path.extension().and_then(|str| str.to_str()) {
                    Ok(DependencyOptions::Git {
                        repo: value.to_string(),
                        branch: None,
                    })
                } else if path.is_dir() {
                    Ok(DependencyOptions::Dir {
                        dir: path.to_path_buf(),
                    })
                } else if path.is_file() {
                    Ok(DependencyOptions::File {
                        file: path.to_path_buf(),
                    })
                } else if let Ok(_url) = url::Url::parse(value) {
                    Ok(DependencyOptions::Url {
                        url: value.to_string(),
                    })
                } else {
                    Err(de::Error::invalid_value(
                        de::Unexpected::Map,
                        &"version or options",
                    ))
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<DependencyOptions, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut repo: Option<String> = None;
                let mut branch: Option<String> = None;
                let mut dir: Option<String> = None;
                let mut file: Option<String> = None;
                let mut version: Option<String> = None;
                let mut url: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "branch" => branch = Some(map.next_value()?),
                        "repo" => repo = Some(map.next_value()?),
                        "dir" => dir = Some(map.next_value()?),
                        "file" => file = Some(map.next_value()?),
                        "version" => version = Some(map.next_value()?),
                        "url" => url = Some(map.next_value()?),
                        _ => unreachable!(),
                    }
                }

                if repo.is_some() {
                    Ok(DependencyOptions::Git {
                        repo: repo.expect("No repo"),
                        branch,
                    })
                } else if dir.is_some() {
                    Ok(DependencyOptions::Dir {
                        dir: Path::new(&dir.expect("No dir")).to_path_buf(),
                    })
                } else if file.is_some() {
                    Ok(DependencyOptions::File {
                        file: Path::new(&file.expect("No file")).to_path_buf(),
                    })
                } else if version.is_some() {
                    Ok(DependencyOptions::Registry {
                        version: VersionReq::parse(&version.expect("No version"))
                            .expect("Not a valid version"),
                    })
                } else if url.is_some() {
                    Ok(DependencyOptions::Url {
                        url: url.expect("No URL"),
                    })
                } else {
                    Err(de::Error::invalid_value(
                        de::Unexpected::Map,
                        &"version or options",
                    ))
                }
            }
        }

        deserializer.deserialize_any(DependencyOptionsVisitor)
    }
}

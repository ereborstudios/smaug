use crate::smaug;
use ::url::Url;
use log::*;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;
use std::process;
use toml::Value;

#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    pub project: Project,
    pub package: Option<Package>,
    pub itch: Option<Itch>,
    pub dependencies: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectConfig {
    pub project: Project,
    pub package: Option<Package>,
    pub itch: Option<Itch>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub author: Option<String>,
    pub icon: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub requires: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Itch {
    pub url: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dependency {
    pub branch: Option<String>,
    pub dir: Option<String>,
    pub name: Option<String>,
    pub repo: Option<String>,
    pub url: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub from: String,
    pub to: String,
    pub require: bool,
}

impl ProjectConfig {
    pub(crate) fn load<P: AsRef<Path>>(path: &P) -> Option<ProjectConfig> {
        let contents = read_to_string(path).unwrap();

        let raw: RawConfig = toml::from_str(&contents).unwrap();
        let config = convert_from_raw(&raw);

        Some(config)
    }
}

fn convert_from_raw(raw: &RawConfig) -> ProjectConfig {
    let dependencies = raw
        .dependencies
        .clone()
        .and_then(|value| load_dependencies(&value))
        .unwrap_or_else(Vec::new);

    ProjectConfig {
        project: raw.project.clone(),
        package: raw.package.clone(),
        itch: raw.itch.clone(),
        dependencies,
    }
}

fn load_dependencies(value: &Value) -> Option<Vec<Dependency>> {
    debug!("Dependencies: {:?}", value);
    return value.as_table().map(|dependencies| {
        dependencies
            .into_iter()
            .map(load_dependency)
            .collect::<Vec<Dependency>>()
    });
}

fn load_dependency((name, value): (&String, &Value)) -> Dependency {
    trace!("Loading dependency {}", name);
    debug!("{:?}", value);

    match value {
        Value::Table(..) => load_dependency_table(name, value),
        Value::String(..) => load_dependency_string(name, value.as_str().unwrap()),
        _ => {
            smaug::print_error(format!("Malformed dependency with name {}", name));
            process::exit(exitcode::DATAERR);
        }
    }
}

pub fn load_dependency_string(name: &str, value: &str) -> Dependency {
    let path = Path::new(value);
    let mut dir: Option<String> = None;
    let mut repo: Option<String> = None;
    let mut file: Option<String> = None;
    let mut url: Option<String> = None;

    match path.extension().and_then(|str| str.to_str()) {
        Some("git") => repo = Some(String::from(value)),
        Some("zip") => {
            let expanded = shellexpand::full(path.to_str().unwrap()).unwrap();
            let expanded = String::from(expanded.chars().as_str());
            let expanded = Path::new(&expanded);
            if expanded.is_file() && zip_extensions::is_zip(&expanded.to_path_buf()) {
                file = Some(String::from(expanded.to_str().unwrap()));
            } else if Url::parse(value).is_ok() {
                url = Some(String::from(value));
            }
        }
        _ => {
            if path.is_dir() {
                dir = Some(String::from(value));
            }
        }
    }

    Dependency {
        name: Some(String::from(name)),
        branch: None,
        dir,
        file,
        repo,
        url,
    }
}

fn load_dependency_table(name: &str, value: &Value) -> Dependency {
    return Dependency {
        name: Some(name.to_string()),
        branch: value
            .get("branch")
            .map(|val| String::from(val.as_str().unwrap())),
        dir: value
            .get("dir")
            .map(|val| String::from(val.as_str().unwrap())),
        file: value
            .get("file")
            .map(|val| String::from(val.as_str().unwrap())),
        repo: value
            .get("repo")
            .map(|val| String::from(val.as_str().unwrap())),
        url: value
            .get("url")
            .map(|val| String::from(val.as_str().unwrap())),
    };
}

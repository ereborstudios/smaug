use crate::smaug;
use log::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use toml::Value;
use url::Url;

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub project: Project,
    pub itch: Option<Itch>,
    pub dependencies: Vec<Dependency>,
    pub files: Vec<File>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub author: Option<String>,
    pub icon: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Itch {
    pub url: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub branch: Option<String>,
    pub dir: Option<String>,
    pub name: Option<String>,
    pub repo: Option<String>,
    pub url: Option<Url>,
    pub file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub from: String,
    pub to: String,
    pub require: bool,
}

impl ProjectConfig {
    pub fn load(path: PathBuf) -> ProjectConfig {
        let raw = fs::read_to_string(path.clone());

        if raw.is_err() {
            smaug::print_error(format!(
                "Could not find configuration at {}.",
                path.to_str().unwrap()
            ));
            process::exit(exitcode::DATAERR);
        }

        let file = raw.unwrap();
        let value = file.parse::<Value>();

        if value.is_err() {
            smaug::print_error(format!("Error parsing {}.", path.to_str().unwrap()));
            smaug::print_error(format!("{}", value.unwrap_err()));
            process::exit(exitcode::DATAERR);
        }

        let config = value.unwrap();

        let project = config.get("project").and_then(load_project).unwrap();
        let itch = config.get("itch").and_then(load_itch);
        let dependencies = config
            .get("dependencies")
            .and_then(load_dependencies)
            .unwrap_or_default();

        let files = config.get("files").and_then(load_files).unwrap_or_default();

        ProjectConfig {
            project,
            itch,
            dependencies,
            files,
        }
    }
}

fn load_project(value: &Value) -> Option<Project> {
    let project = Project {
        author: value
            .get("author")
            .map(|val| String::from(val.as_str().unwrap())),
        icon: value
            .get("icon")
            .map(|val| String::from(val.as_str().unwrap())),
        name: value
            .get("name")
            .map(|val| String::from(val.as_str().unwrap())),
        url: value
            .get("url")
            .map(|val| String::from(val.as_str().unwrap())),
        version: value
            .get("version")
            .map(|val| String::from(val.as_str().unwrap())),
    };

    Some(project)
}

fn load_itch(value: &Value) -> Option<Itch> {
    let itch = Itch {
        url: value
            .get("url")
            .map(|url| String::from(url.as_str().unwrap())),
        username: value
            .get("username")
            .map(|username| String::from(username.as_str().unwrap())),
    };

    Some(itch)
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
    let mut url: Option<Url> = None;

    match path.extension().and_then(|str| str.to_str()) {
        Some("git") => repo = Some(String::from(value)),
        Some("zip") => {
            let expanded = shellexpand::full(path.to_str().unwrap()).unwrap();
            let expanded = String::from(expanded.chars().as_str());
            let expanded = Path::new(&expanded);
            if expanded.is_file() && zip_extensions::is_zip(&expanded.to_path_buf()) {
                file = Some(String::from(expanded.to_str().unwrap()));
            } else if Url::parse(value).is_ok() {
                url = Some(Url::parse(value).unwrap());
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
            .map(|val| Url::parse(val.as_str().unwrap()).unwrap()),
    };
}

fn load_files(value: &Value) -> Option<Vec<File>> {
    return value
        .as_table()
        .map(|files| files.into_iter().map(load_file).collect::<Vec<File>>());
}

fn load_file((from, declaration): (&String, &Value)) -> File {
    match declaration {
        Value::String(..) => File {
            from: from.clone(),
            to: String::from(declaration.as_str().unwrap()),
            require: true,
        },
        Value::Table(..) => {
            let to: &str;

            match declaration.get("path") {
                Some(val) => to = val.as_str().unwrap(),
                None => {
                    smaug::print_error(format!("File {} must include a path value.", from));
                    process::exit(exitcode::DATAERR);
                }
            }

            File {
                from: from.clone(),
                to: String::from(to),
                require: declaration
                    .get("require")
                    .map(|v| v.as_bool().unwrap())
                    .unwrap_or(false),
            }
        }
        _ => unreachable!(),
    }
}

use crate::smaug;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use toml::Value;
use url::Url;

#[derive(Debug)]
pub struct ProjectConfig {
    pub project: Project,
    pub itch: Option<Itch>,
    pub dependencies: Vec<Dependency>,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct Project {
    pub author: Option<String>,
    pub icon: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct Itch {
    pub url: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug)]
pub struct Dependency {
    pub branch: Option<String>,
    pub dir: Option<String>,
    pub name: Option<String>,
    pub repo: Option<String>,
    pub url: Option<Url>,
    pub file: Option<String>,
}

#[derive(Debug)]
pub struct File {
    pub from: String,
    pub to: String,
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
            .unwrap_or(vec![]);

        let files = config.get("files").and_then(load_files).unwrap_or(vec![]);

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
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        icon: value
            .get("icon")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        name: value
            .get("name")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        url: value
            .get("url")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        version: value
            .get("version")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
    };

    return Some(project);
}

fn load_itch(value: &Value) -> Option<Itch> {
    let itch = Itch {
        url: value
            .get("url")
            .and_then(|url| Some(String::from(url.as_str().unwrap()))),
        username: value
            .get("username")
            .and_then(|username| Some(String::from(username.as_str().unwrap()))),
    };

    return Some(itch);
}

fn load_dependencies(value: &Value) -> Option<Vec<Dependency>> {
    return value.as_table().and_then(|dependencies| {
        Some(
            dependencies
                .into_iter()
                .map(load_dependency)
                .collect::<Vec<Dependency>>(),
        )
    });
}

fn load_dependency((name, value): (&String, &Value)) -> Dependency {
    match value {
        Value::Table(..) => return load_dependency_table(name, value),
        Value::String(..) => return load_dependency_string(name, value.as_str().unwrap()),
        _ => {
            smaug::print_error(format!("Malformed dependency with name {}", name));
            process::exit(exitcode::DATAERR);
        }
    }
}

fn load_dependency_string(name: &String, value: &str) -> Dependency {
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

    return Dependency {
        name: Some(String::from(name)),
        branch: None,
        dir: dir,
        file: file,
        repo: repo,
        url: url,
    };
}

fn load_dependency_table(name: &String, value: &Value) -> Dependency {
    return Dependency {
        name: Some(name.clone()),
        branch: value
            .get("branch")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        dir: value
            .get("dir")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        file: value
            .get("file")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        repo: value
            .get("repo")
            .and_then(|val| Some(String::from(val.as_str().unwrap()))),
        url: value
            .get("url")
            .and_then(|val| Some(Url::parse(val.as_str().unwrap()).unwrap())),
    };
}

fn load_files(value: &Value) -> Option<Vec<File>> {
    return value
        .as_table()
        .and_then(|files| Some(files.into_iter().map(load_file).collect::<Vec<File>>()));
}

fn load_file((from, to): (&String, &Value)) -> File {
    return File {
        from: from.clone(),
        to: String::from(to.as_str().unwrap()),
    };
}

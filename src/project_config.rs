use std::fs;
use std::path::PathBuf;
use std::process;
use toml::Value;

#[derive(Debug)]
pub struct ProjectConfig {
  pub project: Project,
  pub itch: Option<Itch>,
  pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Project {
  pub author: Option<String>,
  pub icon: Option<String>,
  pub name: Option<String>,
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
  pub url: Option<String>,
}

impl ProjectConfig {
  pub fn load(path: PathBuf) -> ProjectConfig {
    let file = fs::read_to_string(path).unwrap();
    let value = file.parse::<Value>();

    if value.is_err() {
      println!("{}", value.unwrap_err());
      process::exit(exitcode::DATAERR);
    }

    let config = value.unwrap();

    let project = config.get("project").and_then(load_project).unwrap();
    let itch = config.get("itch").and_then(load_itch);
    let dependencies = config
      .get("dependencies")
      .and_then(load_dependencies)
      .unwrap_or(vec![]);

    ProjectConfig {
      project,
      itch,
      dependencies,
    }
  }
}

fn load_project(value: &Value) -> Option<Project> {
  let project = Project {
    author: value
      .get("author")
      .and_then(|author| Some(String::from(author.as_str().unwrap()))),
    icon: value
      .get("icon")
      .and_then(|icon| Some(String::from(icon.as_str().unwrap()))),
    name: value
      .get("name")
      .and_then(|name| Some(String::from(name.as_str().unwrap()))),
    version: value
      .get("version")
      .and_then(|version| Some(String::from(version.as_str().unwrap()))),
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
  let dependencies = value.as_table().and_then(|dependencies| {
    Some(
      dependencies
        .into_iter()
        .map(load_dependency)
        .collect::<Vec<Dependency>>(),
    )
  });

  return dependencies;
}

fn load_dependency((name, value): (&String, &Value)) -> Dependency {
  return Dependency {
    name: Some(name.clone()),
    branch: value
      .get("branch")
      .and_then(|val| Some(String::from(val.as_str().unwrap()))),
    dir: value
      .get("dir")
      .and_then(|val| Some(String::from(val.as_str().unwrap()))),
    repo: value
      .get("repo")
      .and_then(|val| Some(String::from(val.as_str().unwrap()))),
    url: value
      .get("url")
      .and_then(|val| Some(String::from(val.as_str().unwrap()))),
  };
}

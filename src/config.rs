struct Config {
  project: Project,
  itch: Option<Itch>,
  dependencies: Vec<Dependency>,
}

struct Project {
  author: Option<String>,
  icon: Option<String>,
  name: String,
  version: String,
}

struct Itch {
  url: String,
  username: String,
}

struct Dependency {
  branch: Option<String>,
  dir: Option<String>,
  name: String,
  repo: Option<String>,
  url: Option<String>,
}

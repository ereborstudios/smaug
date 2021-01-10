use crate::project_config::Dependency as DependencyConfig;
use std::path::Path;
use std::path::PathBuf;
use url::Url;

pub enum Dependency {
  Dir {
    path: PathBuf,
  },
  File {
    path: PathBuf,
  },
  Git {
    repo: String,
    branch: Option<String>,
  },
  Url {
    location: Url,
  },
}

impl Dependency {
  pub fn from_config(config: &DependencyConfig) -> Option<Dependency> {
    if config.repo.is_some() {
      let dependency = Dependency::Git {
        repo: config.repo.as_ref().unwrap().clone(),
        branch: config.branch.clone(),
      };

      return Some(dependency);
    } else if config.dir.is_some() {
      let dependency = Dependency::Dir {
        path: Path::new(&config.dir.as_ref().unwrap()).to_path_buf(),
      };

      return Some(dependency);
    } else if config.url.is_some() {
      let dependency = Dependency::Url {
        location: config.url.as_ref().unwrap().clone(),
      };

      return Some(dependency);
    } else if config.file.is_some() {
      let dependency = Dependency::File {
        path: Path::new(&config.file.as_ref().unwrap()).to_path_buf(),
      };

      return Some(dependency);
    } else {
      return None;
    }
  }
}

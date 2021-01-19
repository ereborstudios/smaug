use crate::dependency::Dependency;
use crate::sources::dir_source::DirSource;
use crate::{config::DependencyOptions, sources::git_source::GitSource};
use std::path::PathBuf;

pub trait Source {
    fn install(&self, dependency: &Dependency, path: &PathBuf) -> std::io::Result<()>;
}

pub fn from_dependency_options(options: &DependencyOptions) -> Option<Box<dyn Source>> {
    match options {
        DependencyOptions::Git {
            repo,
            branch,
            rev,
            tag,
        } => Some(Box::new(GitSource {
            repo: repo.clone(),
            branch: branch.clone(),
            rev: rev.clone(),
            tag: tag.clone(),
        })),
        DependencyOptions::Dir { dir: path } => Some(Box::new(DirSource {
            path: path.to_path_buf(),
        })),
        _ => None,
    }
}

use url_source::UrlSource;

use crate::sources::dir_source::DirSource;
use crate::sources::file_source::FileSource;
use crate::{config::DependencyOptions, sources::git_source::GitSource};
use crate::{dependency::Dependency, sources::url_source};
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
        DependencyOptions::File { file: path } => Some(Box::new(FileSource {
            path: path.to_path_buf(),
        })),
        DependencyOptions::Url { url } => Some(Box::new(UrlSource {
            url: url.to_string(),
        })),
        _ => None,
    }
}

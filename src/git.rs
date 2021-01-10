use crate::project_config::Dependency;
use crate::smaug;
use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use std::fs;

pub fn clone(dependency: &Dependency) {
  let destination = smaug::cache_dir().join(dependency.name.as_ref().unwrap());
  fs::remove_dir_all(destination.clone()).unwrap();

  let fetch = FetchOptions::new();
  let checkout = CheckoutBuilder::new();

  RepoBuilder::new()
    .fetch_options(fetch)
    .with_checkout(checkout)
    .branch(dependency.branch.as_ref().unwrap().as_str())
    .clone(&dependency.repo.as_ref().unwrap(), destination.as_path())
    .unwrap();
}

use crate::smaug;
use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use std::fs;
use std::path::Path;

pub fn clone(repository: &str) {
  let destination = smaug::cache_dir().join(repository);
  fs::create_dir_all(destination).unwrap();

  let fetch = FetchOptions::new();
  let checkout = CheckoutBuilder::new();

  RepoBuilder::new()
    .fetch_options(fetch)
    .with_checkout(checkout)
    .clone(&repository, destination);
}

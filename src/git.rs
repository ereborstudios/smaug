use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use std::fs;
use std::path::PathBuf;

pub struct Clone {
  pub repo: String,
  pub branch: Option<String>,
}

impl Clone {
  pub fn clone(&self, destination: &PathBuf) {
    if destination.exists() {
      fs::remove_dir_all(destination.clone()).unwrap();
    }

    let fetch = FetchOptions::new();
    let checkout = CheckoutBuilder::new();

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch);
    builder.with_checkout(checkout);

    if self.branch.is_some() {
      builder.branch(self.branch.as_ref().unwrap().as_str());
    }

    builder.clone(&self.repo, destination.as_path()).unwrap();
  }
}

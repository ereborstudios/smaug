use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use git2::Repository;
use log::*;
use std::fs;
use std::path::PathBuf;

pub struct Clone {
    pub repo: String,
    pub branch: Option<String>,
}

impl Clone {
    pub fn clone(&self, destination: &PathBuf) -> Repository {
        if destination.exists() {
            trace!("Removing directory {}", destination.to_str().unwrap());
            fs::remove_dir_all(destination.clone()).unwrap();
        }

        let fetch = FetchOptions::new();
        let checkout = CheckoutBuilder::new();

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch);
        builder.with_checkout(checkout);

        debug!("Repository: {}", self.repo);
        if self.branch.is_some() {
            debug!("Branch: {}", self.branch.as_ref().unwrap());
            builder.branch(self.branch.as_ref().unwrap().as_str());
        }

        trace!(
            "Cloning git repository to {}",
            destination.to_str().unwrap()
        );
        builder.clone(&self.repo, destination.as_path()).unwrap()
    }
}

use crate::dependency::Dependency;
use crate::resolver::Resolver;
use crate::source::Source;
use crate::sources::dir_source::DirSource;
use git2::build::CheckoutBuilder;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use git2::Oid;
use log::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct GitSource {
    pub repo: String,
    pub branch: Option<String>,
    pub rev: Option<String>,
    pub tag: Option<String>,
}

impl Source for GitSource {
    fn install(
        &self,
        resolver: &mut Resolver,
        dependency: &Dependency,
        path: &PathBuf,
    ) -> std::io::Result<()> {
        let destination = crate::smaug::cache_dir().join(dependency.clone().name);
        trace!(
            "Installing git repository {} to {}",
            self.repo,
            destination.display()
        );

        if destination.exists() {
            trace!("Removing directory {}", destination.to_str().unwrap());
            rm_rf::ensure_removed(destination.clone()).unwrap();
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
        let repository = builder.clone(&self.repo, destination.as_path()).unwrap();

        if self.rev.is_some() {
            let revision = self.rev.clone().unwrap();
            debug!("Revision: {}", revision);
            let mut checkout = CheckoutBuilder::new();
            let object = repository
                .find_object(
                    Oid::from_str(revision.as_str()).expect("Could not find the revision"),
                    None,
                )
                .unwrap();
            repository
                .reset(&object, git2::ResetType::Hard, Some(&mut checkout))
                .unwrap();
        }

        if self.tag.is_some() {
            let tag = self.tag.clone().unwrap();
            let mut checkout = CheckoutBuilder::new();

            let rev = repository
                .revparse_single(&tag)
                .expect("Couldn't parse tag");

            let object = rev.as_tag().expect("Couldn't convert to tag").as_object();

            repository
                .reset(object, git2::ResetType::Hard, Some(&mut checkout))
                .unwrap();
        }

        let git_dir = destination.join(".git");
        rm_rf::ensure_removed(git_dir).expect("Couldn't delete directory");

        let cached = repository.path().parent().expect("No parent dir");

        let result = DirSource {
            path: cached.to_path_buf(),
        }
        .install(resolver, dependency, path);

        rm_rf::ensure_removed(cached).expect("couldn't delete directory");

        result
    }
}

use crate::dependency::Dependency;
use crate::resolver::Resolver;
use crate::source::Source;
use crate::sources::git_source::GitSource;
use log::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RegistrySource {
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryResponse {
    url: String,
    tag: String,
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    repository: RepositoryResponse,
}

#[derive(Debug, Deserialize)]
struct PackageResponse {
    version: VersionResponse,
}

impl Source for RegistrySource {
    fn install(
        &self,
        resolver: &mut Resolver,
        dependency: &Dependency,
        destination: &PathBuf,
    ) -> std::io::Result<()> {
        trace!(
            "Fetching {} version {} from registry",
            dependency.clone().name,
            self.version
        );

        let source = fetch_from_registry(dependency.name.clone(), self.version.clone())?;

        source.install(resolver, dependency, destination)
    }
}

fn fetch_from_registry(name: String, version: String) -> std::io::Result<GitSource> {
    let url = format!(
        "https://api.smaug.dev/packages/{}/versions/{}",
        name, version
    );
    trace!("Fetching from {}", url);
    let response = reqwest::blocking::get(url.as_str());

    match response {
        Err(..) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find package",
        )),
        Ok(response) => parse_response(response, name, version),
    }
}

fn parse_response(
    response: reqwest::blocking::Response,
    name: String,
    version: String,
) -> std::io::Result<GitSource> {
    if response.status().is_success() {
        let package_response: PackageResponse =
            response.json().expect("Couldn't parse registry response");
        Ok(GitSource {
            repo: package_response.version.repository.url,
            tag: Some(package_response.version.repository.tag),
            rev: None,
            branch: None,
        })
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Couldn't fetch {} version {} from repository",
                name, version
            ),
        ))
    }
}

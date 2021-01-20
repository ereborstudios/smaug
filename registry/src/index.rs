use semver::Version;
use smaug::config::Config;
use smaug::dependency::Dependency;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Index {
    pub packages: HashMap<String, PackageIndex>,
}

#[derive(Debug)]
pub struct PackageIndex {
    pub versions: HashMap<Version, Package>,
}

#[derive(Debug)]
pub struct Package {
    pub package: Dependency,
    pub config: Config,
    pub readme: Option<String>,
}

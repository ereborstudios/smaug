use semver::Version;
#[derive(Debug)]
pub struct Project {
    pub authors: Vec<String>,
    pub name: String,
    pub version: Version,
    pub compile_ruby: bool,
}

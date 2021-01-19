#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: semver::VersionReq,
}

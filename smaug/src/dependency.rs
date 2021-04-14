use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

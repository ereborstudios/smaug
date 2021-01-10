pub struct PackageConfig {
  package: Package,
  files: Vec<File>,
}

pub struct Package {
  name: Option<String>,
  version: Option<String>,
  url: Option<String>,
}

pub struct File {
  from: String,
  to: String,
}

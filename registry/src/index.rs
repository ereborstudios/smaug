use semver::Version;
use smaug::config::Config;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use zip::read::ZipArchive;
use zip::read::ZipFile;

#[derive(Debug, Default)]
pub struct Index {
    pub packages: HashMap<String, PackageIndex>,
}

#[derive(Debug, Default)]
pub struct PackageIndex {
    pub versions: HashMap<Version, Package>,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub config: Config,
    pub readme: Option<String>,
    pub filename: String,
}

pub fn new(package_path: &str) -> Index {
    let mut index = Index::default();

    let directory = Path::new(&package_path);
    let files: Vec<PathBuf> = fs::read_dir(directory)
        .expect("Could not read PACKAGE_PATH")
        .map(|e| e.unwrap().path())
        .collect();

    files
        .iter()
        .filter(|f| zip_extensions::is_zip(f))
        .cloned()
        .map(|path| load_package(&path))
        .for_each(|p| index.add_package(p));

    index
}

impl Index {
    pub fn add_package(&mut self, package: Package) {
        let package_index = self
            .packages
            .entry(package.name.clone())
            .or_insert_with(PackageIndex::default);

        package_index
            .versions
            .insert(package.version.clone(), package);
    }
}

fn load_package(path: &PathBuf) -> Package {
    let zipfile = fs::File::open(path).expect("Could not open zip file");
    let mut archive = zip::ZipArchive::new(zipfile).expect("Could not open zip file");
    let config = match archive.by_name("Smaug.toml") {
        Ok(file) => load_config(file, path),
        Err(..) => panic!("Couldn't load package"),
    };

    let package_config = config.package.clone().unwrap();

    let readme = match package_config.readme {
        None => None,
        Some(file) => load_readme(archive, &file),
    };

    Package {
        name: package_config.name,
        version: semver::Version::parse(package_config.version.as_str()).unwrap(),
        config,
        readme,
        filename: path.to_string_lossy().to_string(),
    }
}

fn load_config(mut file: ZipFile, path: &PathBuf) -> Config {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    smaug::config::from_str(&contents, path).unwrap()
}

fn load_readme(mut archive: ZipArchive<fs::File>, path: &str) -> Option<String> {
    match archive.by_name(path) {
        Err(..) => None,
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Some(contents)
        }
    }
}

use directories::ProjectDirs;
use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    return project_dirs().data_dir().to_path_buf();
}

pub fn cache_dir() -> PathBuf {
    return project_dirs().cache_dir().to_path_buf();
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("org", "Erebor Studios", "Smaug").expect("No project directories found.")
}

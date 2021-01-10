use directories::ProjectDirs;
use std::path::PathBuf;
use std::process;

pub fn data_dir() -> PathBuf {
  return project_dirs().data_dir().to_path_buf();
}

pub fn cache_dir() -> PathBuf {
  return project_dirs().cache_dir().to_path_buf();
}

fn project_dirs() -> ProjectDirs {
  let project_dirs = ProjectDirs::from("org", "Erebor Studios", "Smaug");

  match project_dirs {
    Some(dirs) => return dirs,
    None => {
      println!("No project directories found");
      process::exit(exitcode::OSFILE);
    }
  }
}

use directories::ProjectDirs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

pub fn dragonruby_platform() -> &'static str {
  #[cfg(target_os = "windows")]
  return "windows-amd64";

  #[cfg(target_os = "macos")]
  return "macos";

  return "linux-amd64";
}

pub fn dragonruby_directory() -> PathBuf {
  let destination: &Path;
  let directory_name = format!("dragonruby-{}", dragonruby_platform());
  let project_dirs = ProjectDirs::from("org", "Erebor Studios", "Smaug");

  match project_dirs {
    Some(ref dirs) => {
      destination = dirs.data_dir();
    }
    None => {
      println!("No data directories found");
      process::exit(exitcode::OSFILE);
    }
  }

  return destination.join(directory_name);
}

pub fn ensure_installed() {
  if !dragonruby_directory().exists() {
    println!("Install DragonRuby with \"smaug install PATH_TO_DRAGONRUBY_ZIP\"");
    process::exit(exitcode::UNAVAILABLE);
  }
}

use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;
use zip_extensions::zip_extract;

pub fn unzip(source: &Path, destination: &Path) {
  if destination.is_dir() {
    fs::remove_dir_all(destination).unwrap();
  }
  zip_extract(&source.to_path_buf(), &destination.to_path_buf()).unwrap();

  let files = fs::read_dir(destination)
    .unwrap()
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()
    .unwrap();

  let file_count = files.iter().count();
  let maybe_dir = files.first().unwrap();

  if file_count == 1 && maybe_dir.is_dir() {
    for entry in WalkDir::new(maybe_dir) {
      let entry = entry.unwrap();
      let entry = entry.path();

      let new_path = entry
        .to_str()
        .unwrap()
        .replace(maybe_dir.to_str().unwrap(), destination.to_str().unwrap());
      let new_path = Path::new(&new_path);

      if entry.is_file() {
        fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        fs::copy(entry, new_path).unwrap();
      }
    }

    fs::remove_dir_all(maybe_dir).unwrap();
  }
}

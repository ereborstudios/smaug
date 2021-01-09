use crate::dragonruby;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;

pub fn install(matches: &&clap::ArgMatches) {
  let filename: &str = matches.value_of("FILE").unwrap();
  let path = Path::new(filename);
  let destination: PathBuf;

  if path.exists() {
    destination = setup_destination();
    extract(path, &destination);
  } else {
    println!("The file {} does not exist", path.to_str().unwrap());
    process::exit(exitcode::NOINPUT);
  }
}

fn setup_destination() -> PathBuf {
  let path = dragonruby::dragonruby_directory();
  let destination = Path::parent(&path).unwrap();

  let result = fs::create_dir_all(destination)
    .and_then(|()| fs::remove_dir_all(destination).and_then(|()| fs::create_dir_all(destination)));

  match result {
    Ok(()) => return destination.to_path_buf(),
    Err(error) => {
      println!(
        "Error creating directory at {}\n{}",
        destination.to_str().unwrap(),
        error
      );
      process::exit(exitcode::DATAERR);
    }
  }
}

fn extract(source: &Path, destination: &Path) {
  let file = fs::File::open(&source).unwrap();

  let mut archive = zip::ZipArchive::new(file).unwrap();

  println!("Installing DragonRuby");

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = match file.enclosed_name() {
      Some(path) => destination.join(path).to_owned(),
      None => continue,
    };

    {
      let comment = file.comment();
      if !comment.is_empty() {
        println!("File {} comment: {}", i, comment);
      }
    }

    if (&*file.name()).ends_with('/') {
      fs::create_dir_all(&outpath).unwrap();
    } else {
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }
      let mut outfile = fs::File::create(&outpath).unwrap();
      io::copy(&mut file, &mut outfile).unwrap();
    }

    // Get and Set permissions
    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;

      if let Some(mode) = file.unix_mode() {
        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
      }
    }
  }

  println!(
    "Extracting from {} to {}",
    source.to_str().unwrap(),
    destination.to_str().unwrap()
  );
}

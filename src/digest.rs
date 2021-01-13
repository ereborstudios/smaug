// use blake2::{Blake2b, Digest};
// use std::path::Path;
// use std::{fs, io};

// pub fn file(path: &Path) -> io::Result<String> {
//     let mut file = fs::File::open(path)?;
//     let mut hasher = Blake2b::new();
//     io::copy(&mut file, &mut hasher)?;
//     let hash = hasher.finalize();

//     Ok(format!("{:x}", hash))
// }

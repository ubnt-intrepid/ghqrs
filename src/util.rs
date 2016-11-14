use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use shellexpand;

#[allow(dead_code)]
pub struct PushDir {
  orig_path: PathBuf,
}

impl PushDir {
  #[allow(dead_code)]
  pub fn enter<P: AsRef<Path>>(new_path: P) -> PushDir {
    let curr_dir = env::current_dir().unwrap();
    env::set_current_dir(new_path).unwrap();
    PushDir { orig_path: curr_dir }
  }
}

impl Drop for PushDir {
  fn drop(&mut self) {
    let _ = env::set_current_dir(&self.orig_path);
  }
}

// Read the content of a file in `paths`
pub fn read_file_if_exists(paths: &[&str]) -> Result<Option<String>, io::Error> {
  for path in paths {
    // expand the candidate of path.
    let path = shellexpand::full(path).unwrap().into_owned();
    let path = Path::new(&path);

    if path.exists() && path.is_file() {
      let mut content = String::new();
      return File::open(path)
        .and_then(|ref mut file| file.read_to_string(&mut content))
        .and(Ok(Some(content)));
    }
  }
  Ok(None)
}

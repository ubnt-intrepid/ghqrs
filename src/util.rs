#![allow(dead_code)]

use std::env;
use std::path::{Path, PathBuf};

pub struct PushDir {
  orig_path: PathBuf,
}

impl PushDir {
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

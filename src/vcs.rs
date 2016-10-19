#![allow(dead_code)]

use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use url::Url;

pub struct Git;

impl Git {
  pub fn clone(url: Url, dest: &Path, depth: Option<i32>) -> Result<i32, io::Error> {
    let depth = depth.map(|depth| format!("--depth={}", depth));
    let mut args = vec!["clone", url.as_str(), dest.to_str().unwrap()];
    if let Some(ref depth) = depth {
      args.push(&depth);
    }

    wait_exec("git", args.as_slice(), None)
  }

  pub fn update(dest: &Path) -> Result<i32, io::Error> {
    wait_exec("git", &["pull", "--ff-only"], Some(dest))
  }
}

pub struct Mercurial;

impl Mercurial {
  pub fn clone(url: Url, dest: &Path, _: Option<i32>) -> Result<i32, io::Error> {
    wait_exec("hg", &["clone", url.as_str(), dest.to_str().unwrap()], None)
  }

  pub fn update(path: &Path) -> Result<i32, io::Error> {
    wait_exec("hg", &["pull", "--update"], Some(path))
  }
}


fn wait_exec(cmd: &str, args: &[&str], curr_dir: Option<&Path>) -> Result<i32, io::Error> {
  let mut command = Command::new(cmd);
  command.args(args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());
  if let Some(curr_dir) = curr_dir {
    command.current_dir(curr_dir);
  }

  let mut child = try!(command.spawn());
  child.wait()
    .and_then(|st| st.code().ok_or(io::Error::new(io::ErrorKind::Other, "")))
}

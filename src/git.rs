extern crate url;

use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use url::Url;

fn launch_git(args: &[&str], curr_dir: Option<&Path>) -> Result<i32, io::Error> {
  let mut cmd = Command::new("git");
  if let Some(curr_dir) = curr_dir {
    cmd.current_dir(curr_dir);
  }

  let mut process = try!(cmd.args(args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn());

  process.wait()
    .and_then(|st| st.code().ok_or(io::Error::new(io::ErrorKind::Other, "")))
}

pub fn clone(url: Url, dest: &Path, depth: Option<i32>) -> Result<i32, io::Error> {
  let depth = depth.map(|depth| format!("--depth={}", depth));

  let mut args = vec!["clone", url.as_str(), dest.to_str().unwrap()];
  if let Some(ref depth) = depth {
    args.push(&depth);
  }

  launch_git(args.as_slice(), None)
}


pub fn pull(dest: &Path) -> Result<i32, io::Error> {
  launch_git(&["pull", "--ff-only"], Some(dest))
}

pub fn config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8_lossy(&output.stdout[0..len - 1]).into_owned()
}

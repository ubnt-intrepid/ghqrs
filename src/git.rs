extern crate url;

use std::path::Path;
use std::process::Command;
use url::Url;
use util::PushDir;


pub fn clone(url: Url, dest: &Path, depth: Option<i32>) -> Result<String, String> {
  let mut args =
    vec!["clone".to_owned(), url.as_str().to_owned(), dest.to_str().unwrap().to_owned()];
  if let Some(depth) = depth {
    args.push(format!("--depth={}", depth));
  }

  let output = try!(Command::new("git")
    .args(args.as_slice())
    .output()
    .map_err(|e| e.to_string()));
  if !output.status.success() {
    return Err(format!("failed to clone git repository: {:?}", output.stderr));
  }
  String::from_utf8(output.stdout).map_err(|e| e.to_string())
}


pub fn pull(dest: &Path) -> Result<String, String> {
  let pushd = PushDir::enter(dest);

  let output = try!(Command::new("git")
    .args(&["pull", "--ff-only"])
    .output()
    .map_err(|e| e.to_string()));
  if !output.status.success() {
    return Err(format!("failed to pull git repository: {:?}", output.stderr));
  }

  drop(pushd);
  String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

pub fn config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8(Vec::from(&output.stdout[0..len - 1])).unwrap()
}

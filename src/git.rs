extern crate url;

use std::path::Path;
use std::process::Command;
use url::Url;
use util::PushDir;


pub fn clone(url: Url, dest: &Path, shallow: bool) -> Result<String, String> {
  let mut args = vec!["clone", url.as_str(), dest.to_str().unwrap()];
  if shallow {
    args.extend(&["--depth", "1"]);
  }

  let output = try!(Command::new("git")
    .args(&args[..])
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
    .args(&["pull"])
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

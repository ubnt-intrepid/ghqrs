extern crate url;

use std::path::Path;
use std::process::Command;
use url::Url;
use util::PushDir;


#[allow(unreachable_code)]
pub fn clone(url: Url, dest: &Path, shallow: bool) {
  println!("clone: {:?} -> {:?} ({})",
           url,
           dest,
           if shallow { "Shallow" } else { "" });
  return;

  let mut args = vec!["clone", url.as_str(), dest.to_str().unwrap()];
  if shallow {
    args.extend(&["--depth", "1"]);
  }

  let output = Command::new("git")
    .args(&args[..])
    .output()
    .expect("failed to clone repository");
  if !output.status.success() {
    panic!("git clone failed");
  }
}


#[allow(unreachable_code)]
pub fn pull(dest: &Path) {
  println!("pull: {:?}", dest);
  return;

  let pushd = PushDir::enter(dest);

  let output = Command::new("git")
    .args(&["pull"])
    .output()
    .expect("failed to clone repository");
  if !output.status.success() {
    panic!("git pull failed");
  }

  drop(pushd);
}

pub fn config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8(Vec::from(&output.stdout[0..len - 1])).unwrap()
}

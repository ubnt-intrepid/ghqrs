extern crate url;

use std::path::Path;
use std::process::{Command, Stdio};
use url::Url;

fn launch_git(args: &[String], curr_dir: Option<&Path>) -> Result<i32, String> {
  let mut cmd = Command::new("git");
  if let Some(curr_dir) = curr_dir {
    cmd.current_dir(curr_dir);
  }

  let mut process = try!(cmd.args(args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .map_err(|e| e.to_string()));

  process.wait()
    .map_err(|e| e.to_string())
    .and_then(|st| st.code().ok_or("".to_owned()))
}

pub fn clone(url: Url, dest: &Path, depth: Option<i32>) -> Result<i32, String> {
  let mut args: Vec<String> =
    vec!["clone".to_owned(), url.as_str().to_owned(), dest.to_str().unwrap().to_owned()];
  if let Some(depth) = depth {
    args.push(format!("--depth={}", depth));
  }

  launch_git(args.as_slice(), None)
}


pub fn pull(dest: &Path) -> Result<i32, String> {
  launch_git(&["pull".to_owned(), "--ff-only".to_owned()], Some(dest))
}

pub fn config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8(Vec::from(&output.stdout[0..len - 1])).unwrap()
}

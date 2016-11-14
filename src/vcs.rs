use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub enum VCS {
  Git,
  Svn,
  Hg,
  Darcs,
}

impl FromStr for VCS {
  type Err = ();

  fn from_str(s: &str) -> Result<VCS, ()> {
    match s {
      "git" => Ok(VCS::Git),
      "svn" => Ok(VCS::Svn),
      "hg" => Ok(VCS::Hg),
      "darcs" => Ok(VCS::Darcs),
      _ => Err(()),
    }
  }
}

pub fn detect<P: AsRef<Path>>(path: P) -> Option<VCS> {
  vec![".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .find(|&vcs| path.as_ref().join(vcs).exists())
    .map(|e| format!("{}", &e[1..]))
    .and_then(|s| s.parse().ok())
}

pub struct Git;

impl Git {
  pub fn clone(url: &Url, dest: &Path, depth: Option<i32>) -> Result<i32, io::Error> {
    let depth = depth.map(|depth| format!("--depth={}", depth));
    let mut args = vec!["clone", url.as_str(), dest.to_str().unwrap()];
    if let Some(ref depth) = depth {
      args.push(&depth);
    }

    wait_exec("git", args.as_slice(), None)
  }

  #[allow(dead_code)]
  pub fn update(dest: &Path) -> Result<i32, io::Error> {
    wait_exec("git", &["pull", "--ff-only"], Some(dest))
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

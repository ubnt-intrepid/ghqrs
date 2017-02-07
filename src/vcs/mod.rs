pub mod git;
pub mod hg;
pub mod svn;

use std::io;
use std::path::Path;
use std::str::FromStr;
use url::Url;

pub enum Status {
  Git(git::Status),
  Hg(hg::Status),
  Svn(svn::Status),
}

impl Status {
  pub fn prompt(&self, fallback: bool) -> String {
    let (vcs, prompt_str) = match *self {
      Status::Git(ref s) => ("git", s.prompt(fallback)),
      Status::Hg(ref s) => ("hg", s.prompt(fallback)),
      Status::Svn(ref s) => ("svn", s.prompt(fallback)),
    };

    format!("[{}]({})", vcs, prompt_str)
  }
}

pub fn current_status(wd: &Path) -> Result<Option<Status>, io::Error> {
  if wd.join(".git").exists() {
    git::current_status().map(|s| s.map(|s| Status::Git(s)))
  } else if wd.join(".hg").exists() {
    hg::current_status().map(|s| s.map(|s| Status::Hg(s)))
  } else if wd.join(".svn").exists() {
    svn::current_status().map(|s| s.map(|s| Status::Svn(s)))
  } else if let Some(parent) = wd.parent() {
    current_status(parent)
  } else {
    Ok(None)
  }
}

pub trait Prompt {
  fn prompt(&self, fallback: bool) -> String;
}



#[derive(Debug)]
pub enum VCS {
  Git,
  Svn,
  Hg,
  Darcs,
}

impl VCS {
  #[allow(dead_code)]
  pub fn detect<P: AsRef<Path>>(path: P) -> Option<VCS> {
    vec![".git", ".svn", ".hg", "_darcs"]
      .into_iter()
      .find(|&vcs| path.as_ref().join(vcs).exists())
      .map(|e| format!("{}", &e[1..]))
      .and_then(|s| s.parse().ok())
  }

  pub fn clone_repository(&self, url: &Url, dest: &Path) -> io::Result<()> {
    git::clone(url, dest, None).map(|_| ())
  }
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

pub fn is_vcs_subdir(path: &Path) -> bool {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .any(|vcs| path.join("..").join(vcs).exists())
}

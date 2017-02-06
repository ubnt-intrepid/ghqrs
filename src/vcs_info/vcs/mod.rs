mod git;
mod hg;
mod svn;

use std::io;
use std::path::Path;

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

trait Prompt {
  fn prompt(&self, fallback: bool) -> String;
}

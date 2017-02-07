use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use regex::Regex;
use url::Url;

use vcs::Prompt;
use util::*;

#[derive(Default)]
pub struct Status {
  branch: String,
  ahead_by: usize,
  behind_by: usize,
  upstream: String,
  index: Option<DiffInfo>,
  working: Option<DiffInfo>,
  untracked: usize,
  stash_count: usize,
}

#[derive(Default)]
struct DiffInfo {
  added: usize,
  modified: usize,
  renamed: usize,
  copied: usize,
  deleted: usize,
  unmerged: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
enum DescribeStyle {
  Default,
  Contains,
  Branch,
  Describe,
}

fn get_git_dir(wd: &Path) -> Option<PathBuf> {
  get_local_or_parent_path(wd, ".git")
}

fn get_local_or_parent_path(wd: &Path, path: &str) -> Option<PathBuf> {
  if wd.join(path).exists() {
    Some(wd.join(path).to_path_buf())
  } else if let Some(parent) = wd.parent() {
    get_local_or_parent_path(&parent, path)
  } else {
    None
  }
}

fn get_branch() -> io::Result<String> {
  let git_dir = if let Some(git_dir) = get_git_dir(&env::current_dir().unwrap()) {
    git_dir
  } else {
    return Ok(String::new());
  };

  let mut r = String::new();
  let mut b;
  let mut c = String::new();

  if git_dir.join("rebase-merge/interactive").exists() {
    // interactive rebase
    r = "|REBASE-i".to_owned();
    b = read_content(git_dir.join("rebase-merge/head-name"))?;

  } else if git_dir.join("rebase-merge").exists() {
    // rebase-merge
    r = "|REBASE-m".to_owned();
    b = read_content(git_dir.join("rebase-merge/head-name"))?;

  } else {
    if git_dir.join("rebase-apply").exists() {
      if git_dir.join("rebase-apply/rebasing").exists() {
        r = "|REBASE".to_owned();
      } else if git_dir.join("rebase-apply/applying").exists() {
        r = "|AM".to_owned();
      } else {
        r = "|AM/REBASE".to_owned();
      }
    } else if git_dir.join("MERGE_HEAD").exists() {
      r = "|MERGING".to_owned();
    } else if git_dir.join("CHERRY_PICK_HEAD").exists() {
      r = "|CHERRY-PICKING".to_owned();
    } else if git_dir.join("BISECT_LOG").exists() {
      r = "|BISECTING".to_owned();
    }

    // trying symbolic-ref
    let _ref = get_lines("git", &["symbolic-ref", "HEAD", "-q"])
      ?
      .into_iter()
      .next();
    if let Some(br) = _ref {
      b = br;
    } else {
      // get tag or SHA1-hash
      let hash = get_tag_or_hash(&git_dir, DescribeStyle::Default)?;
      b = format!("({})", hash);
    }
  }

  // inside Git directory?
  if get_lines("git", &["rev-parse", "--is-inside-git-dir"])
    ?
    .into_iter()
    .next()
    .unwrap() == "true" {
    if get_lines("git", &["rev-parse", "--is-bare-repository"])
      ?
      .into_iter()
      .next()
      .unwrap() == "true" {
      c = "BARE:".to_owned();
    } else {
      b = "GIT_DIR!".to_owned();
    }
  }

  Ok(format!("{}{}{}", c, b.replace("refs/heads/", ""), r))
}


fn get_tag_or_hash(git_dir: &Path, style: DescribeStyle) -> io::Result<String> {
  // trying describe
  let describe = match style {
    DescribeStyle::Contains => get_lines("git", &["describe", "--contains", "HEAD"])?,
    DescribeStyle::Branch => get_lines("git", &["describe", "--contains", "--all", "HEAD"])?,
    DescribeStyle::Describe => get_lines("git", &["describe", "HEAD"])?,
    DescribeStyle::Default => get_lines("git", &["tag", "--points-at", "HEAD"])?,
  };
  if let Some(b) = describe.into_iter().next() {
    return Ok(b);
  }

  // falling back on parsing HEAD
  let ref_ = if git_dir.join("HEAD").exists() {
    // reading from .git/HEAD
    read_content(git_dir.join("HEAD"))?

  } else {
    // trying git rev-parse
    get_lines("git", &["rev-parse", "HEAD"])
      ?
      .into_iter()
      .next()
      .unwrap_or_default()
  };

  let re = Regex::new(r"ref: (?P<ref>.+)").unwrap();

  if let Some(cap) = re.captures(&ref_) {
    Ok(cap.name("ref").unwrap().to_owned())

  } else if ref_.len() >= 7 {
    Ok(format!("{}...", &ref_[7..]))

  } else {
    Ok("unknown".to_owned())
  }
}

fn get_stash_count() -> Result<usize, io::Error> {
  if communicate("git", &["rev-parse", "--verify", "--quiet", "refs/stash"]).is_err() {
    return Ok(0);
  }

  let (stdout, stderr, _) = communicate("git",
                                        &["log",
                                          "--format=\"%%gd: %%gs\"",
                                          "-g",
                                          "--first-parent",
                                          "-m",
                                          "refs/stash",
                                          "--"])?;
  if stderr.contains("fatal") {
    return Err(io::Error::new(io::ErrorKind::Other, stderr.as_str()));
  }

  Ok(stdout.split("\n").count() - 1)
}


pub fn current_status() -> io::Result<Option<Status>> {
  let mut status = Status::default();

  let lines = get_lines("git",
                        &["-c", "color.status=false", "status", "--short", "--branch"])?;

  // get branch information.
  if let Some(line_branch) = lines.iter().next() {
    let re1 = Regex::new(r"^## (?P<branch>\S+?)(?:\.\.\.(?P<upstream>\S+))?(?: \[(?:ahead (?P<ahead>\d+))?(?:, )?(?:behind (?P<behind>\d+))?\])?$").unwrap();
    let re2 = Regex::new(r"^## Initial commit on (?P<branch>\S+)$").unwrap();

    if let Some(caps) = re1.captures(line_branch) {
      status.branch = caps.name("branch").unwrap_or_default().to_owned();
      status.upstream = caps.name("upstream").unwrap_or_default().to_owned();
      status.ahead_by = caps.name("ahead").unwrap_or("0").parse::<usize>().unwrap_or(0);
      status.behind_by = caps.name("behind").unwrap_or("0").parse::<usize>().unwrap_or(0);

    } else if let Some(caps) = re2.captures(line_branch) {
      status.branch = caps.name("branch").unwrap_or_default().to_owned();
    }

  } else {
    return Ok(None);
  }
  if status.branch == "" {
    status.branch = get_branch()?;
  }


  // parse status line.
  let mut index = DiffInfo::default();
  let mut working = DiffInfo::default();

  let re = Regex::new(r"^(?P<index>[^#])(?P<working>.) (.*?)(?: -> (.*))?$").unwrap();

  for ref caps in (&lines[1..]).iter().filter_map(|ref line| re.captures(line)) {
    match caps.name("index") {
      Some("A") => index.added += 1,
      Some("M") => index.modified += 1,
      Some("R") => index.renamed += 1,
      Some("C") => index.copied += 1,
      Some("D") => index.deleted += 1,
      Some("U") => index.unmerged += 1,
      _ => (),
    }

    match caps.name("working") {
      Some("A") => working.added += 1,
      Some("M") => working.modified += 1,
      Some("R") => working.renamed += 1,
      Some("C") => working.copied += 1,
      Some("D") => working.deleted += 1,
      Some("U") => working.unmerged += 1,
      Some("?") => status.untracked += 1,
      _ => (),
    }
  }

  if index.added > 0 || index.modified > 0 || index.deleted > 0 || index.unmerged > 0 {
    status.index = Some(index);
  }

  if working.added > 0 || working.modified > 0 || working.deleted > 0 || working.unmerged > 0 {
    status.working = Some(working);
  }

  // collect stash count.
  status.stash_count = get_stash_count()?;

  Ok(Some(status))
}

impl Prompt for Status {
  fn prompt(&self, fallback: bool) -> String {
    let mut ret = String::new();

    // show branch information
    ret += &self.branch;
    if self.upstream == "" {
      // no upstream branch
    } else if self.behind_by == 0 && self.ahead_by == 0 {
      // aligned with remote
      ret += if fallback { " =" } else { " ≡" };
    } else if self.behind_by > 0 && self.ahead_by > 0 {
      // both behind and ahead of remote
      ret += if fallback { " AB" } else { " ↕" };
    } else if self.ahead_by > 0 {
      // ahead of remote
      ret += if fallback { " A" } else { " ↑" };
    } else if self.behind_by > 0 {
      // behind remote
      ret += if fallback { " B" } else { " ↓" };
    } else {
      // unknown state
      ret += " ?";
    }

    if let Some(ref s) = self.index {
      ret += " |I";
      ret += &format!(" +{}", s.added);
      ret += &format!(" ~{}", s.modified + s.renamed + s.copied);
      ret += &format!(" -{}", s.deleted);
      ret += &format!(" !{}", s.unmerged);
    }

    if let Some(ref s) = self.working {
      ret += " |W";
      ret += &format!(" +{}", s.added);
      ret += &format!(" ~{}", s.modified + s.renamed + s.copied);
      ret += &format!(" -{}", s.deleted);
      ret += &format!(" !{}", s.unmerged);
    }

    if self.untracked > 0 {
      ret += &format!(" |? {}", self.untracked);
    }

    if self.stash_count > 0 {
      ret += &format!(" |S {}", self.stash_count);
    }

    ret
  }
}

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

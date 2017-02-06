// original implementation: https://github.com/JeremySkinner/posh-hg

use std::io;
use super::Prompt;
use vcs_info::util::*;
use regex::Regex;

#[derive(Default)]
pub struct Status {
  tags: Vec<String>,
  commit: String,
  branch: String,
  behind: bool,
  head_count: usize,
  multiple_heads: bool,
  active: String,
  rev: String,
  diff: Option<Diff>,
}

#[derive(Default)]
struct Diff {
  added: usize,
  modified: usize,
  deleted: usize,
  untracked: usize,
  missing: usize,
  renamed: usize,
}

pub fn current_status() -> io::Result<Option<Status>> {
  let mut status = Status::default();

  let mut has_diff = false;
  let mut diff = Diff::default();

  if false {
    // getFileStatus == false
    let re1 = Regex::new(r"tag:\s*(.*)").unwrap();
    let re2 = Regex::new(r"changeset:\s*(\S)").unwrap();
    let re3 = Regex::new(r"changeset:\s*(\S*)").unwrap();

    let lines = get_lines("hg", &["parent"])?;
    for line in lines {
      if let Some(m) = re1.captures(&line).and_then(|c| c.at(1)) {
        status.tags = m.replace("(empty repository)", "")
          .split(" ")
          .filter_map(|l| {
            let trimmed = l.trim();
            if trimmed.len() > 0 {
              Some(trimmed.to_owned())
            } else {
              None
            }
          })
          .collect();
      } else if let Some(m) = re2.captures(&line).and_then(|c| c.at(1)) {
        status.commit = m.to_owned();
      }
    }

    status.branch = get_lines("hg", &["branch"])?.into_iter().nth(0).unwrap();
    status.behind = true;
    status.head_count = 0;

    let lines = get_lines("hg", &["heads", &status.branch])?;
    for line in lines {
      if let Some(m) = re3.captures(&line).and_then(|c| c.at(1)) {
        if status.commit == m {
          status.behind = false;
        }
        status.head_count += 1;
        if status.head_count > 1 {
          status.multiple_heads = true;
        }
      }
    }

  } else {
    let re1 = Regex::new(r"parent: (\S*) ?(.*)").unwrap();
    let re2 = Regex::new(r"branch: ([\S ]*)").unwrap();
    let re3 = Regex::new(r"update: (\d+)").unwrap();
    let re4 = Regex::new(r"pmerge: (\d+) pending").unwrap();
    let re5 = Regex::new(r"commit: (.*)").unwrap();
    let re6 = Regex::new(r"(\d+) (modified|added|removed|deleted|unknown|renamed)").unwrap();

    let lines = get_lines("hg", &["summary"])?;
    for line in lines {
      if let Some(cap) = re1.captures(&line) {
        if cap.len() > 2 {
          status.commit = cap.at(1).unwrap().to_owned();
          status.tags = cap.at(2)
            .unwrap()
            .replace("(empty repository)", "")
            .split(" ")
            .filter_map(|l| {
              let trimmed = l.trim();
              if trimmed.len() > 0 {
                Some(trimmed.to_owned())
              } else {
                None
              }
            })
            .collect();
        }
      } else if let Some(m) = re2.captures(&line).and_then(|c| c.at(1)) {
        status.branch = m.to_owned();
      } else if re3.is_match(&line) {
        status.behind = true;
      } else if re4.is_match(&line) {
        status.behind = true
      } else if let Some(m) = re5.captures(&line).and_then(|c| c.at(1)) {
        for token in m.split(",") {
          if let Some(cap) = re6.captures(&token) {
            if cap.len() > 2 {
              if let Some(num) = cap.at(1).and_then(|s| s.parse::<usize>().ok()) {
                match cap.at(2) {
                  Some("modified") => {
                    has_diff = true;
                    diff.modified = num
                  }
                  Some("added") => {
                    has_diff = true;
                    diff.added = num
                  }
                  Some("removed") => {
                    has_diff = true;
                    diff.deleted = num
                  }
                  Some("deleted") => {
                    has_diff = true;
                    diff.missing = num
                  }
                  Some("unknown") => {
                    has_diff = true;
                    diff.untracked = num
                  }
                  Some("renamed") => {
                    has_diff = true;
                    diff.renamed = num
                  }
                  _ => (),
                }
              }
            }
          }
        }
      }
    }
  }

  status.diff = if has_diff { Some(diff) } else { None };

  // getBookmarkStatus
  if true {
    let lines = get_lines("hg", &["bookmarks"])?;
    for line in lines {
      if line.trim().starts_with("*") {
        status.active = line.split(" ").nth(2).unwrap().to_owned();
      }
    }
  }

  let lines = get_lines("hg",
                        &["log", "-r", ".", "--template", "{rev}:{node|short}"])?;
  status.rev = lines[0].clone();

  Ok(Some(status))
}

impl Prompt for Status {
  fn prompt(&self, _: bool) -> String {
    let mut ret = String::new();

    // show branch
    ret += &self.branch;

    // show diff
    if let Some(ref diff) = self.diff {
      ret += &format!("|+{}", diff.added);
      ret += &format!(" ~{}", diff.modified);
      ret += &format!(" x{}", diff.deleted);
      ret += &format!(" ?{}", diff.untracked);
      ret += &format!(" m{}", diff.missing);
      ret += &format!(" c{}", diff.renamed);
    }

    // show tags
    if self.tags.len() > 0 || self.active != "" {
      ret += "|";
      if self.active != "" {
        ret += &self.active;
        if self.tags.len() > 0 {
          ret += " ";
        }
      }

      for (i, ref tag) in self.tags.iter().enumerate() {
        ret += tag;
        if i < self.tags.len() - 1 {
          ret += ", ";
        }
      }
    }

    // show patches
    if false {
      // always disabled
      if let Ok(patches) = get_patches("") {
        if patches.all.len() > 0 {
          ret += "|";

          for (i, ref a) in patches.applied.iter().enumerate() {
            ret += a;
            if i < patches.applied.len() - 1 {
              ret += ", ";
            }
          }
          if patches.unapplied.len() > 0 {
            ret += ", ";
          }
          for (i, ref u) in patches.unapplied.iter().enumerate() {
            ret += u;
            if i < patches.unapplied.len() - 1 {
              ret += ", ";
            }
          }
        }
      }
    }

    // show revision
    if self.rev != "" {
      ret += &format!(" <{}>", self.rev);
    }

    ret
  }
}

struct Patches {
  all: Vec<String>,
  applied: Vec<String>,
  unapplied: Vec<String>,
}

fn get_patches(filter: &str) -> io::Result<Patches> {
  let mut applied = Vec::new();
  let mut unapplied = Vec::new();

  let lines = get_lines("hg", &["qseries", "-v"])?;
  for line in lines {
    let status = line.split(" ").nth(1).unwrap().to_owned();
    let name = line.split(" ").nth(2).unwrap().to_owned();
    if status == "A" {
      applied.push(name);
    } else {
      unapplied.push(name);
    }
  }

  let mut all: Vec<_> = applied.iter().chain(unapplied.iter()).cloned().collect();
  if filter != "" {
    all = all.into_iter().filter(|a| a.starts_with(filter)).collect();
  };

  Ok(Patches {
    all: all,
    applied: applied,
    unapplied: unapplied,
  })
}

use std::io;
use vcs::Prompt;
use util::*;

#[derive(Default)]
struct Diff {
  untracked: usize,
  ignored: usize,
  added: usize,
  modified: usize,
  replaced: usize,
  deleted: usize,
  missing: usize,
  conflicted: usize,
  obstructed: usize,
}

#[derive(Default)]
pub struct Status {
  diff: Option<Diff>,
  external: usize,
  incoming: usize,
  branch: String,
  revision: String,
  incoming_revision: usize,
}

pub fn current_status() -> io::Result<Option<Status>> {
  let mut s = Status::default();

  let mut has_diff = false;
  let mut diff = Diff::default();

  // TODO: add '-u' option
  let lines = get_lines("svn", &["status", "--ignore-externals"])?;
  for line in lines {
    if line.starts_with("Status") {
      s.incoming_revision = line.replace("Status against revision:", "").trim().parse().unwrap();
    } else {
      match line.chars().next() {
        Some('A') => {
          has_diff = true;
          diff.added += 1
        }
        Some('C') => {
          has_diff = true;
          diff.conflicted += 1
        }
        Some('D') => {
          has_diff = true;
          diff.deleted += 1
        }
        Some('I') => {
          has_diff = true;
          diff.ignored += 1
        }
        Some('M') => {
          has_diff = true;
          diff.modified += 1
        }
        Some('R') => {
          has_diff = true;
          diff.replaced += 1
        }
        Some('?') => {
          has_diff = true;
          diff.untracked += 1
        }
        Some('!') => {
          has_diff = true;
          diff.missing += 1
        }
        Some('~') => {
          has_diff = true;
          diff.obstructed += 1
        }
        Some('X') => s.external += 1,
        _ => (),
      }

      match line.chars().nth(4) {
        Some('X') => s.external += 1,
        _ => (),
      }
      match line.chars().nth(6) {
        Some('C') => {
          has_diff = true;
          diff.conflicted += 1
        }
        _ => (),
      }
      match line.chars().nth(8) {
        Some('*') => s.incoming += 1,
        _ => (),
      }
    }
  }

  s.diff = if has_diff { Some(diff) } else { None };

  let branch_info = get_branch_info()?;
  s.branch = branch_info.0;
  s.revision = branch_info.1;

  Ok(Some(s))
}

fn get_branch_info() -> io::Result<(String, String)> {
  let info = get_lines("svn", &["info"])?;

  let url = info[3].replace("Relative URL: ^/", "");
  let revision = info[6].replace("Revision: ", "");

  let path_bits: Vec<_> = url.split("/")
    .filter_map(|line| if line.trim().len() > 0 {
      Some(line)
    } else {
      None
    })
    .collect();

  let mut branch = String::new();
  if path_bits.len() > 0 {
    if path_bits[0] == "trunk" {
      branch = "trunk".to_owned();
    } else if path_bits.len() > 1 &&
              (path_bits[0].contains("branches") || path_bits[0].contains("tags")) {
      branch = path_bits[1].to_owned();
    }
  }

  Ok((branch, revision))
}

impl Prompt for Status {
  fn prompt(&self, _: bool) -> String {
    let mut ret = String::new();

    ret.push_str(&self.branch);
    ret.push_str(&format!("@{}", self.revision));

    if let Some(ref diff) = self.diff {
      ret.push_str(&format!("|+{}", diff.added));
      ret.push_str(&format!(" ~{}", diff.modified + diff.replaced));
      ret.push_str(&format!(" -{}", diff.deleted));
      ret.push_str(&format!(" ?{}", diff.untracked));
      ret.push_str(&format!(" !{}", diff.missing));
      ret.push_str(&format!(" C{}", diff.conflicted + diff.obstructed));
    }
    if self.incoming > 0 {
      ret.push_str(&format!("|In{}@{}", self.incoming, self.incoming_revision));
    }

    if self.external > 0 {
      ret.push_str(&format!("|Ex{}", self.external));
    }

    ret
  }
}

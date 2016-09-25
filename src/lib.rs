extern crate walkdir;
extern crate git2;

use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
struct Repository {
  root: String,
  path: String,
}

fn get_local_repositories(filter: Box<Fn(&str) -> bool>, unique: bool) -> Vec<Repository> {
  let mut dst = Vec::new();

  let roots = get_local_repos_roots();
  for root in roots {
    for entry in WalkDir::new(&root).follow_links(true).into_iter().filter_map(|e| e.ok()) {
      let path = format!("{}", entry.path().display()).replace(&format!("{}/", root), "");
      if entry.depth() == 3 {

        let entry = vec![".git", ".svn", ".hg", "_darcs"].into_iter().find(|&e| {
          let mut buf = PathBuf::from(format!("{}", entry.path().display()));
          if !filter(buf.file_name().unwrap().to_str().unwrap()) {
            return false;
          }
          buf.push(e);
          buf.exists()
        });

        if entry.is_some() {
          dst.push(Repository {
            root: root.clone(),
            path: path,
          });
        }
      }
    }
  }

  dst
}

pub fn command_list(exact: bool, fullpath: bool, unique: bool, query: Option<String>) -> i32 {
  let filter: Box<Fn(&str) -> bool> = {
    if let Some(query) = query {
      if exact {
        Box::new(move |s: &str| s == query)
      } else {
        Box::new(move |s: &str| s.contains(&query))
      }
    } else {
      Box::new(|_| true)
    }
  };

  let repos = get_local_repositories(filter, unique);
  for repo in repos {
    if fullpath {
      let mut repo_path = std::path::PathBuf::from(repo.root);
      repo_path.push(repo.path);
      println!("{}", repo_path.display());
    } else {
      println!("{}", repo.path);
    }
  }

  0
}

pub fn command_get(project: String) -> i32 {
    // resolve to url
    // clone repository
    0
}

fn git_clone(url: &str, dest: &str) {
  let output = std::process::Command::new("git")
    .args(&["clone", url, dest])
    .output()
    .expect("failed to clone repository");
  if !output.status.success() {
    panic!("git clone failed");
  }
}

fn git_config(key: &str) -> String {
  let output = std::process::Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8(Vec::from(&output.stdout[0..len - 1])).unwrap()
}

fn get_local_repos_roots() -> Vec<String> {
  let mut local_repo_roots;

  let env_root: String = std::env::var("GHQ_ROOT").unwrap_or("".to_owned());
  if env_root == "" {
    local_repo_roots = vec![git_config("ghq.root")];
  } else {
    local_repo_roots = env_root.split(":").map(|s| s.to_owned()).collect();
  }

  if local_repo_roots.len() == 0 {
    let mut ghq_path = std::env::home_dir().unwrap();
    ghq_path.push(".ghq");
    local_repo_roots.push(format!("{}", ghq_path.display()));
  }

  assert!(local_repo_roots.len() >= 1);

  local_repo_roots
}

pub fn command_root(all: bool) -> i32 {
  let roots = get_local_repos_roots();
  if all {
    for root in roots {
      println!("{}", root);
    }
  } else {
    println!("{}", roots[0]);
  }
  0
}

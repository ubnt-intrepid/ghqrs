extern crate walkdir;
extern crate regex;
extern crate url;

mod git;
mod util;
mod remote;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;


pub fn command_get(projects: Vec<String>, skip_pull: bool, shallow: bool) -> i32 {
  for project in projects {
    let repo = remote::RemoteRepository::new(project.as_str());
    let dest = repo.local_path(&get_local_repos_roots()[0]);

    git::clone_or_pull(repo.url(), dest.as_path(), skip_pull, shallow);
  }
  0
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

  for repo in get_local_repositories(filter) {
    if fullpath {
      let mut repo_path = PathBuf::from(repo.root);
      repo_path.push(repo.path);
      println!("{}", repo_path.display());
    } else if unique {
      let repo_name = Path::new(&repo.path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
      println!("{}", repo_name);
    } else {
      println!("{}", repo.path);
    }
  }

  0
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



#[derive(Debug)]
struct Repository {
  root: String,
  path: String,
}

fn get_local_repositories(filter: Box<Fn(&str) -> bool>) -> Vec<Repository> {
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

fn get_local_repos_roots() -> Vec<String> {
  let mut local_repo_roots;

  let env_root: String = std::env::var("GHQ_ROOT").unwrap_or("".to_owned());
  if env_root == "" {
    local_repo_roots = vec![git::config("ghq.root")];
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

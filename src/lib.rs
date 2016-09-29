extern crate walkdir;
extern crate regex;
extern crate url;

mod vcs;
mod util;
mod remote;

use std::path::Path;
use walkdir::WalkDir;
use remote::RemoteRepository;
use std::process::Command;


pub fn command_get(projects: Vec<String>, pull: bool, depth: Option<i32>) -> i32 {
  for project in projects {
    let repo = RemoteRepository::parse(project.as_str()).unwrap();
    repo.clone_or_pull(&get_local_repos_roots()[0], pull, depth).unwrap();
  }
  0
}

pub fn command_list(exact: bool, format: &str, query: Option<String>) -> i32 {
  let filter: Box<Fn(&Repository) -> bool>;
  if let Some(query) = query {
    if exact {
      filter = Box::new(move |repo: &Repository| repo.project_name() == query);
    } else {
      filter = Box::new(move |repo: &Repository| repo.contains(&query));
    }
  } else {
    filter = Box::new(|_| true);
  }

  for repo in get_local_repositories().into_iter().filter(|ref repo| filter(repo)) {
    let path = match format {
      "full" => repo.absolute_path(),
      "unique" => repo.unique_path(),
      _ => repo.path,
    };
    println!("{}", path);
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
  vcs: String,
  root: String,
  path: String,
}

impl Repository {
  #[cfg(windows)]
  fn absolute_path(&self) -> String {
    let repo_path = Path::new(&self.root).join(&self.path);
    format!("{}", repo_path.display()).replace("/", "\\")
  }

  #[cfg(not(windows))]
  fn absolute_path(&self) -> String {
    let repo_path = Path::new(&self.root).join(&self.path);
    format!("{}", repo_path.display())
  }

  fn unique_path(&self) -> String {
    Path::new(&self.path)
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }

  fn project_name(&self) -> String {
    Path::new(&self.path).file_name().unwrap().to_str().unwrap().to_owned()
  }

  fn contains(&self, query: &str) -> bool {
    let target: Vec<&str> = self.path.split("/").collect();
    let target: Vec<&str> = target.into_iter().rev().take(2).collect();
    format!("{}/{}", target[1], target[0]).contains(query)
  }
}

fn get_local_repositories() -> Vec<Repository> {
  let mut dst = Vec::new();

  let roots = get_local_repos_roots();
  for root in roots {
    for entry in WalkDir::new(&root)
      .follow_links(true)
      .min_depth(2)
      .max_depth(3)
      .into_iter()
      .filter_map(|e| e.ok()) {

      let path = format!("{}", entry.path().display())
        .replace(&format!("{}{}", root, std::path::MAIN_SEPARATOR), "");

      let vcs = vec![".git", ".svn", ".hg", "_darcs"]
        .into_iter()
        .find(|&vcs| entry.path().join(vcs).exists())
        .map(|e| format!("{}", &e[1..]));

      if vcs.is_some() {
        dst.push(Repository {
          vcs: vcs.unwrap(),
          root: root.clone(),
          path: path.replace("\\", "/"),
        });
      }
    }
  }

  dst
}

fn get_local_repos_roots() -> Vec<String> {
  let mut local_repo_roots;

  let env_root: String = std::env::var("GHQ_ROOT").unwrap_or("".to_owned());
  if env_root == "" {
    local_repo_roots = vec![get_config("ghq.root")];
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

fn get_config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8_lossy(&output.stdout[0..len - 1]).into_owned()
}

use std;
use std::path::Path;

use config;
use walkdir::WalkDir;


#[derive(Debug)]
pub struct Repository {
  vcs: String,
  root: String,
  path: String,
}

impl Repository {
  #[cfg(windows)]
  pub fn absolute_path(&self) -> String {
    let repo_path = Path::new(&self.root).join(&self.path);
    format!("{}", repo_path.display()).replace("/", "\\")
  }

  #[cfg(not(windows))]
  pub fn absolute_path(&self) -> String {
    let repo_path = Path::new(&self.root).join(&self.path);
    format!("{}", repo_path.display())
  }

  pub fn unique_path(&self) -> String {
    Path::new(&self.path)
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }

  pub fn relative_path(&self) -> String {
    self.path.clone()
  }

  pub fn project_name(&self) -> String {
    Path::new(&self.path).file_name().unwrap().to_str().unwrap().to_owned()
  }

  pub fn contains(&self, query: &str) -> bool {
    let target: Vec<&str> = self.path.split("/").collect();
    let target: Vec<&str> = target.into_iter().rev().take(2).collect();
    format!("{}/{}", target[1], target[0]).contains(query)
  }
}


pub fn get_local_repositories() -> Vec<Repository> {
  let mut dst = Vec::new();

  let roots = config::get_roots();
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

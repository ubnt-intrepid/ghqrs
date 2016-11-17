use std::path::{Path, PathBuf};
use walkdir::{WalkDir, WalkDirIterator};

use config::Config;
use repository::*;
use error::GhqError;
use vcs;


pub struct Workspace {
  repos: Vec<(PathBuf, Vec<Repository>)>,
}

impl Workspace {
  pub fn new(config: Config) -> Workspace {
    let repos = config.roots
      .iter()
      .map(Path::new)
      .map(|root| (root.to_owned(), collect_local_repos(&root)))
      .collect();

    Workspace { repos: repos }
  }

  pub fn iter_roots(&self) -> Vec<&Path> {
    self.repos.iter().map(|root| root.0.as_path()).collect()
  }

  pub fn filter_repos<F, T>(&self, f: F) -> Vec<T>
    where F: Fn(&Repository, &Path) -> T
  {
    self.repos
      .iter()
      .flat_map(|repo| repo.1.iter().map(|r| f(r, &repo.0)).collect::<Vec<_>>())
      .collect()
  }

  // clone a remote repository into the workspace.
  pub fn clone_from(&self, s: &str, root: Option<&str>) -> Result<(), GhqError> {
    // get the path of root directory
    let root = root.map(Path::new)
      .or(self.iter_roots().into_iter().next())
      .unwrap_or(Path::new(""));

    if !Path::new(&root).exists() {
      println!("The root directory does not exist: {}", root.display());
      return Ok(());
    }

    Repository::from_remote(s)?.clone_into(&root)
  }
}

fn collect_local_repos<P: AsRef<Path>>(root: P) -> Vec<Repository> {
  WalkDir::new(&root)
    .follow_links(true)
    .into_iter()
    .filter_entry(|entry| !is_vcs_component(entry.path()))
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| relative_path(entry.path(), root.as_ref()).ok())
    .filter_map(|path| Repository::from_local(&path).ok())
    .collect()
}

// make relative path from `root`
fn relative_path(path: &Path, root: &Path) -> Result<PathBuf, GhqError> {
  Ok(path.strip_prefix(root)?.to_path_buf())
}

fn is_vcs_component(path: &Path) -> bool {
  if let Some(path) = path.parent() {
    [".git", ".svn", ".hg", "_dacrs"].into_iter().any(|v| vcs::is_vcs_subdir(&path.join(v)))
  } else {
    false
  }
}

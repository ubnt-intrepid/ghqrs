use std::fs;
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

  pub fn roots(&self) -> Vec<&Path> {
    self.repos.iter().map(|root| root.0.as_path()).collect()
  }

  pub fn default_root(&self) -> Option<&Path> {
    self.repos.iter().next().map(|root| root.0.as_path())
  }

  pub fn filter_repos<F, T>(&self, f: F) -> Vec<T>
    where F: Fn(&Repository, &Path) -> T
  {
    let f = &f;
    self.repos
      .iter()
      .flat_map(|&(ref root, ref repos)| repos.iter().map(move |r| f(r, root)))
      .collect()
  }

  // clone a remote repository into the workspace.
  pub fn clone_from(&self, s: &str) -> Result<(), GhqError> {
    // get root directory
    let root = self.default_root()
      .ok_or("Cannot get root directory of the workspace")?;
    if !root.exists() {
      fs::create_dir_all(root)?;
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

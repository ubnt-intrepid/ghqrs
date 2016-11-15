use std::collections::BTreeMap;
use std::path::Path;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use config;
use vcs;
use remote;
use error::GhqError;


pub struct Workspace {
  config: config::Config,
  repos: BTreeMap<String, Vec<Repository>>,
}

impl Workspace {
  pub fn new(config: config::Config) -> Workspace {
    let mut repos = BTreeMap::new();

    for root in &config.roots {
      let repo = WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| !is_vcs_subdir(entry))
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| Repository::new(&entry).ok())
        .collect();
      repos.insert(root.to_owned(), repo);
    }

    Workspace {
      config: config,
      repos: repos,
    }
  }

  pub fn show_roots(&self, all: bool) {
    if all {
      for root in &self.config.roots {
        println!("{}", root);
      }
    } else if let Some(ref root) = self.config.roots.iter().next() {
      println!("{}", root);
    }
  }

  pub fn show_repositories(&self) {
    for (_, repos) in &self.repos {
      for repo in repos {
        println!("{}", repo.path);
      }
    }
  }

  // clone a remote repository into the workspace.
  pub fn clone_from(&self, s: &str, root: Option<&str>) -> Result<(), GhqError> {
    // get the path of root directory
    let root = root.or(self.config.roots.iter().next().map(|s| s.as_str())).unwrap_or("");
    if !Path::new(root).exists() {
      println!("The root directory does not exist: {}", root);
      return Ok(());
    }

    let (url, host, path) = remote::parse_token(s)?;
    let dest = Path::new(root).join(host).join(path);

    if dest.exists() {
      println!("The target has already existed: {}", dest.display());
      return Ok(());
    }

    println!("clone '{}' into '{}'", url.as_str(), dest.display());
    vcs::Git::clone(&url, dest.as_path(), None).map(|_| ()).map_err(Into::into)
  }
}

fn is_vcs_subdir(entry: &DirEntry) -> bool {
  [".git", ".svn", ".hg", "_darcs"]
    .into_iter()
    .any(|vcs| entry.path().join("..").join(vcs).exists())
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Repository {
  vcs: vcs::VCS,
  path: String,
}

impl Repository {
  fn new(entry: &DirEntry) -> Result<Repository, ()> {
    let vcs = vcs::detect(entry.path()).ok_or(())?;
    let path = entry.path().to_str().ok_or(())?;
    Ok(Repository {
      vcs: vcs,
      path: path.to_owned(),
    })
  }
}

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, MAIN_SEPARATOR};
use walkdir::WalkDir;

use config::Config;
use vcs;
use remote;

pub struct Workspace {
  config: Config,
  repos: BTreeMap<String, Vec<Repository>>,
}

impl Workspace {
  pub fn init() -> Result<Workspace, io::Error> {
    let config = try!(Config::load());

    let mut repos = BTreeMap::new();
    for root in &config.roots {
      let repo = get_repositories(&root);
      repos.insert(root.to_owned(), repo);
    }

    Ok(Workspace {
      config: config,
      repos: repos,
    })
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

  pub fn clone_repository(&self, query: &str) {
    let url = remote::make_remote_url(query).unwrap();
    if let Some(ref root) = self.config.roots.iter().next() {
      let repo = remote::RemoteRepository::new(url).unwrap();
      repo.clone(&root, None).unwrap();
    }
  }

  pub fn show_repositories(&self, format: ListFormat) {
    for (_, repos) in &self.repos {
      for repo in repos {
        let path = match format {
          ListFormat::Default => repo.relative_path(),
          ListFormat::FullPath => repo.absolute_path(),
          ListFormat::Unique => repo.unique_path(),
        };
        println!("{}", path);
      }
    }
  }
}

fn get_repositories(root: &str) -> Vec<Repository> {
  let mut repos = Vec::new();
  for entry in WalkDir::new(&root)
    .follow_links(true)
    .min_depth(2)
    .max_depth(3)
    .into_iter()
    .filter_map(|e| e.ok()) {

    let path = format!("{}", entry.path().display())
      .replace(&format!("{}{}", root, MAIN_SEPARATOR), "");

    if let Some(vcs) = vcs::detect(entry.path()) {
      let repo = Repository {
        vcs: vcs,
        root: root.to_owned(),
        path: path.replace("\\", "/"),
      };
      repos.push(repo);
    }
  }

  repos
}

// output format
pub enum ListFormat {
  // relative path from host directory
  // e.g. github.com/hoge/fuga
  Default,

  // absolute path
  // e.g. /home/hoge/github.com/hoge/fuga or C:\Users\hoge\github.com\hoge\fuga
  FullPath,

  // only project name
  // e.g. fuga
  Unique,
}

impl<'a> From<&'a str> for ListFormat {
  fn from(s: &str) -> ListFormat {
    match s {
      "full" => ListFormat::FullPath,
      "unique" => ListFormat::Unique,
      _ => ListFormat::Default,
    }
  }
}

#[derive(Debug)]
pub struct Repository {
  vcs: vcs::VCS,
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
}

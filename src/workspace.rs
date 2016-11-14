use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use walkdir::WalkDir;
use url::Url;

use config;
use vcs;
use remote;

pub struct Workspace {
  config: config::Config,
  repos: BTreeMap<String, Vec<Repository>>,
}

impl Workspace {
  pub fn new(config: config::Config) -> Workspace {
    let mut repos = BTreeMap::new();
    for root in &config.roots {
      let repo = get_repositories(&root);
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

  pub fn clone_repository(&self, query: &str) {
    let url = remote::make_remote_url(query).unwrap();
    if let Some(ref root) = self.config.roots.iter().next() {
      let repo = RemoteRepository::new(url).unwrap();
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


struct RemoteRepository {
  protocol: String,
  base_url: String,
  user: String,
  project: String,
}

impl RemoteRepository {
  pub fn new(url: Url) -> Result<RemoteRepository, String> {
    let protocol = url.scheme().to_owned();
    let base_url = try!(url.host_str().ok_or("cannot retrieve host information".to_owned()))
      .to_owned();

    let paths: Vec<_> = try!(url.path_segments().ok_or("failed to split URL".to_owned()))
      .map(ToOwned::to_owned)
      .collect();
    let user = paths[0].clone();
    let repo = paths[1].trim_right_matches(".git").to_owned();

    Ok(RemoteRepository {
      protocol: protocol,
      base_url: base_url,
      user: user,
      project: repo,
    })
  }

  fn url(&self) -> Url {
    Url::parse(&format!("{}://{}/{}/{}.git",
                        self.protocol,
                        self.base_url,
                        self.user,
                        self.project))
      .unwrap()
  }

  fn local_path(&self, root: &str) -> PathBuf {
    let mut dest = PathBuf::from(root);
    dest.push(&self.base_url);
    dest.push(&self.user);
    dest.push(&self.project);
    dest
  }

  pub fn clone(&self, root: &str, depth: Option<i32>) -> Result<(), io::Error> {
    let url = self.url();
    let dest = self.local_path(root);
    if dest.exists() {
      println!("exists: {}", dest.display());
    } else {
      println!("clone: {} -> {}", url.as_str(), dest.display());
      try!(vcs::Git::clone(url, dest.as_path(), depth));
    }
    Ok(())
  }
}

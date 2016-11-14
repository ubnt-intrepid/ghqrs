use std::collections::BTreeMap;
use std::io;
use std::path::{Path, MAIN_SEPARATOR};
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
      repo.clone(&root).unwrap();
    }
  }

  pub fn show_repositories(&self) {
    for (_, repos) in &self.repos {
      for repo in repos {
        let path = repo.absolute_path();
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


#[derive(Debug)]
pub struct Repository {
  vcs: vcs::VCS,
  root: String,
  path: String,
}

impl Repository {
  pub fn absolute_path(&self) -> String {
    let repo_path = Path::new(&self.root).join(&self.path);
    format!("{}", repo_path.display()).replace("/", &format!("{}", MAIN_SEPARATOR))
  }
}

struct RemoteRepository {
  url: Url,
  subpath: Vec<String>,
}

impl RemoteRepository {
  pub fn new(url: Url) -> Result<RemoteRepository, String> {
    // parse URL
    let protocol = url.scheme().to_owned();
    let base_url = try!(url.host_str().ok_or("cannot retrieve host information".to_owned()))
      .to_owned();
    let paths: Vec<_> = try!(url.path_segments().ok_or("failed to split URL".to_owned()))
      .map(ToOwned::to_owned)
      .collect();
    let user = paths[0].clone();
    let repo = paths[1].trim_right_matches(".git").to_owned();

    let url = Url::parse(&format!("{}://{}/{}/{}.git", protocol, base_url, user, repo)).unwrap();
    let subpath = vec![base_url, user, repo];
    Ok(RemoteRepository {
      url: url,
      subpath: subpath,
    })
  }

  pub fn clone(&self, root: &str) -> Result<(), io::Error> {
    let mut dest = Path::new(root).to_path_buf();
    for ref subpath in &self.subpath {
      dest.push(subpath);
    }

    if dest.exists() {
      println!("exists: {}", dest.display());
      return Ok(());
    }

    println!("clone: {} -> {}", self.url.as_str(), dest.display());
    vcs::Git::clone(&self.url, dest.as_path(), None).map(|_| ())
  }
}

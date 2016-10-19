use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use config;
use vcs;
use url::{Url, ParseError};
use walkdir::WalkDir;


#[derive(Debug)]
pub struct LocalRepository {
  vcs: String,
  root: String,
  path: String,
}

pub struct RemoteRepository {
  protocol: String,
  base_url: String,
  user: String,
  project: String,
}


impl LocalRepository {
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

  // pub fn project_name(&self) -> String {
  //   Path::new(&self.path).file_name().unwrap().to_str().unwrap().to_owned()
  // }

  // pub fn contains(&self, query: &str) -> bool {
  //   let target: Vec<&str> = self.path.split("/").collect();
  //   let target: Vec<&str> = target.into_iter().rev().take(2).collect();
  //   format!("{}/{}", target[1], target[0]).contains(query)
  // }
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

  pub fn clone_or_pull(&self, root: &str, pull: bool, depth: Option<i32>) -> Result<(), io::Error> {
    let url = self.url();
    let dest = self.local_path(root);
    if dest.exists() {
      if !pull {
        println!("exists: {}", dest.display());
      } else {
        println!("update: {}", dest.display());
        try!(vcs::Git::update(dest.as_path()));
      }
    } else {
      println!("clone: {} -> {}", url.as_str(), dest.display());
      try!(vcs::Git::clone(url, dest.as_path(), depth));
    }
    Ok(())
  }
}


pub fn get_local_repositories() -> BTreeMap<String, Vec<LocalRepository>> {
  let mut dst = BTreeMap::new();

  let roots = config::get_roots();
  for root in roots {
    let mut repos = Vec::new();
    for entry in WalkDir::new(&root)
      .follow_links(true)
      .min_depth(2)
      .max_depth(3)
      .into_iter()
      .filter_map(|e| e.ok()) {

      let path = format!("{}", entry.path().display())
        .replace(&format!("{}{}", root, MAIN_SEPARATOR), "");

      let vcs = vec![".git", ".svn", ".hg", "_darcs"]
        .into_iter()
        .find(|&vcs| entry.path().join(vcs).exists())
        .map(|e| format!("{}", &e[1..]));

      if vcs.is_some() {
        let repo = LocalRepository {
          vcs: vcs.unwrap(),
          root: root.clone(),
          path: path.replace("\\", "/"),
        };
        repos.push(repo);
      }
    }

    dst.insert(root, repos);
  }

  dst
}

pub fn make_remote_url(project: &str) -> Result<Url, ParseError> {
  Url::parse(project).or_else(|_| make_remote_url_from_relative(project))
}

fn make_remote_url_from_relative(project: &str) -> Result<Url, ParseError> {
  let paths: Vec<_> = project.split("/").collect();

  let (host, user, repo) = match paths.len() {
    3 => (paths[0], paths[1], paths[2]),
    2 => ("github.com", paths[0], paths[1]),
    1 => ("github.com", paths[0], paths[0]),
    _ => {
      panic!("'{}' is an unsupported pattern to resolve remote URL.",
             project)
    }
  };

  Url::parse(&format!("https://{}/{}/{}.git", host, user, repo))
}

#[cfg(test)]
mod test {
  use super::make_remote_url;

  #[test]
  fn user_project() {
    let url = make_remote_url("hoge/fuga").unwrap();
    assert_eq!(url.as_str(), "https://github.com/hoge/fuga.git");
  }

  #[test]
  fn domain_user_project() {
    let url = make_remote_url("github.com/hoge/fuga").unwrap();
    assert_eq!(url.as_str(), "https://github.com/hoge/fuga.git");
  }

  #[test]
  fn only_project_name() {
    let url = make_remote_url("fuga").unwrap();
    assert_eq!(url.as_str(), "https://github.com/fuga/fuga.git");
  }

  #[test]
  fn repository_url() {
    let url = make_remote_url("https://gitlab.com/funga-/pecopeco.git").unwrap();
    assert_eq!(url.as_str(), "https://gitlab.com/funga-/pecopeco.git");
  }
}

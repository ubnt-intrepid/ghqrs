use std::path::{Path, PathBuf};
use url::Url;
use error::GhqError;
use vcs::VCS;


#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
const KNOWN_HOSTS: &'static [(&'static str, usize)] = &[
    ("github.com", 2)
  , ("gist.github.com", 1)
  , ("bitbucket.org", 2)
  , ("gitlab.com", 2)
];


#[derive(Debug)]
pub struct Repository {
  url: Option<Url>,
  vcs: VCS,
  host: String,
  path: String,
}

impl Repository {
  pub fn from_local<P: AsRef<Path>>(path: P) -> Result<Repository, GhqError> {
    let path = path.as_ref().to_string_lossy().replace("\\", "/");
    let splitted: Vec<_> = path.splitn(2, '/').collect();
    if splitted.len() < 2 {
      return Err("invalid path").map_err(Into::into);
    }

    let host = splitted[0].to_owned();

    // check depth
    let path_depth = splitted[1].split("/").count();
    let host_depth = match host.as_str() {
      "gist.github.com" => 1,
      _ => 2, // github.com, bitbucket.org, gitlab.com, ...
    };
    if path_depth != host_depth {
      Err("wrong depth in path")?;
    }

    let path = Vec::from(&splitted[1..]).join("/");

    Ok(Repository {
      url: None,
      vcs: VCS::Git,
      host: host,
      path: path,
    })
  }

  pub fn from_remote(s: &str) -> Result<Repository, GhqError> {
    if let Ok(url) = Url::parse(s) {
      let host = url.host_str().ok_or("cannot retrieve host information")?.to_owned();
      let path = url.path().trim_left_matches("/").trim_right_matches(".git").to_owned();
      // TODO: check if given URL is valid

      Ok(Repository {
        url: Some(url),
        vcs: VCS::Git,
        host: host,
        path: path,
      })

    } else {
      let path: Vec<_> = s.split("/").collect();
      let (host, path) = match path.len() {
        0 => Err("unsupported pattern to resolve remote URL")?,
        1 => ("github.com".to_owned(), vec![path[0], path[0]]),
        2 => ("github.com".to_owned(), vec![path[0], path[1]]),
        _ => (path[0].to_owned(), Vec::from(&path[1..])),
      };

      let url = Url::parse(&format!("{}://{}/{}.git",
                                    "https",
                                    host,
                                    path.iter().take(2).cloned().collect::<Vec<_>>().join("/")))?;

      Ok(Repository {
        url: Some(url),
        vcs: VCS::Git,
        host: host,
        path: path.join("/"),
      })
    }
  }

  pub fn local_path(&self, root: &str) -> PathBuf {
    Path::new(&Path::new(root)
        .join(&self.host)
        .join(&self.path)
        .to_string_lossy()
        .replace("\\", "/"))
      .to_path_buf()
  }

  pub fn clone_into<P: AsRef<Path>>(&self, root: P) -> Result<(), GhqError> {
    if let Some(ref url) = self.url {
      let dest = root.as_ref().join(&self.host).join(&self.path);

      if dest.exists() {
        println!("The target has already existed: {}", dest.display());
        return Ok(());
      }

      println!("clone '{}' into '{}'", url.as_str(), dest.display());
      self.vcs.clone_repository(url, dest.as_path())?;
    }

    Ok(())
  }
}


#[cfg(test)]
mod test_from_local {
  use super::Repository;

  #[test]
  fn case1() {
    let repo = Repository::from_local("github.com/hoge/fuga").unwrap();
    assert!(repo.url.is_none());
    assert_eq!(repo.host, "github.com");
    assert_eq!(repo.path, "hoge/fuga");
  }

  #[test]
  fn case2() {
    let repo = Repository::from_local("gist.github.com/0bacdbefa19f").unwrap();
    assert!(repo.url.is_none());
    assert_eq!(repo.host, "gist.github.com");
    assert_eq!(repo.path, "0bacdbefa19f");
  }
}


#[cfg(test)]
mod test_from_remote {
  use super::Repository;

  macro_rules! def_test {
    ($name:ident, $s:expr, $url:expr, $host:expr, $path:expr) => {
      #[test]
      fn $name() {
        let repo = Repository::from_remote($s).unwrap();
        assert_eq!(repo.url.unwrap().as_str(), $url);
        assert_eq!(repo.host, $host);
        assert_eq!(repo.path, $path);
      }
    }
  }

  def_test!(user_project,
            "hoge/fuga",
            "https://github.com/hoge/fuga.git",
            "github.com",
            "hoge/fuga");

  def_test!(domain_user_project,
            "github.com/hoge/fuga",
            "https://github.com/hoge/fuga.git",
            "github.com",
            "hoge/fuga");

  def_test!(only_project_name,
            "fuga",
            "https://github.com/fuga/fuga.git",
            "github.com",
            "fuga/fuga");

  def_test!(repository_url,
            "https://gitlab.com/funga-/pecopeco.git",
            "https://gitlab.com/funga-/pecopeco.git",
            "gitlab.com",
            "funga-/pecopeco");

  def_test!(long_path,
            "github.com/hoge/fuga/foo/a/b/c",
            "https://github.com/hoge/fuga.git",
            "github.com",
            "hoge/fuga/foo/a/b/c");
}

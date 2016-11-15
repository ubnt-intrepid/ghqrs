use std::path::Path;
use url::Url;
use error::GhqError;
use vcs;


#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
const KNOWN_HOSTS: &'static [(&'static str, usize)] = &[
    ("github.com", 2)
  , ("gist.github.com", 1)
  , ("bitbucket.org", 2)
  , ("gitlab.com", 2)
];


#[allow(dead_code)]
#[derive(Debug)]
pub struct Repository {
  vcs: vcs::VCS,
  pub path: String,
}

impl Repository {
  pub fn from_local(path: &Path) -> Result<Repository, ()> {
    let vcs = vcs::detect(path).ok_or(())?;
    let path = path.to_str().map(ToOwned::to_owned).ok_or(())?;
    Ok(Repository {
      vcs: vcs,
      path: path,
    })
  }
}


pub fn parse_token(s: &str) -> Result<(Url, String, String), GhqError> {
  let url = make_remote_url(s)?;

  let host = url.host_str().ok_or("cannot retrieve host information").map(ToOwned::to_owned)?;
  let path = url.path().trim_left_matches("/").trim_right_matches(".git").to_owned();

  Ok((url, host, path))
}

fn make_remote_url(s: &str) -> Result<Url, GhqError> {
  if let Ok(url) = Url::parse(s) {
    return Ok(url);
  }

  let path: Vec<_> = s.split("/").collect();
  let path = match path.len() {
    0 => return Err("unsupported pattern to resolve remote URL").map_err(Into::into),
    1 => vec!["github.com", path[0], path[0]],
    2 => vec!["github.com", path[0], path[1]],
    _ => path,
  };

  Url::parse(&format!("{}://{}.git", "https", path.join("/"))).map_err(Into::into)
}


#[cfg(test)]
mod test_parse_token {
  use super::parse_token;

  fn assert_helper(s: &str, _url: &str, _host: &str, _path: &str) {
    let (url, host, path) = parse_token(s).unwrap();
    assert_eq!(url.as_str(), _url);
    assert_eq!(host, _host);
    assert_eq!(path, _path);
  }

  #[test]
  fn user_project() {
    assert_helper("hoge/fuga",
                  "https://github.com/hoge/fuga.git",
                  "github.com",
                  "hoge/fuga");
  }

  #[test]
  fn domain_user_project() {
    assert_helper("github.com/hoge/fuga",
                  "https://github.com/hoge/fuga.git",
                  "github.com",
                  "hoge/fuga");
  }

  #[test]
  fn only_project_name() {
    assert_helper("fuga",
                  "https://github.com/fuga/fuga.git",
                  "github.com",
                  "fuga/fuga");
  }

  #[test]
  fn repository_url() {
    assert_helper("https://gitlab.com/funga-/pecopeco.git",
                  "https://gitlab.com/funga-/pecopeco.git",
                  "gitlab.com",
                  "funga-/pecopeco");
  }

  #[test]
  fn long_path() {
    assert_helper("github.com/hoge/fuga/foo/a/b/c",
                  "https://github.com/hoge/fuga/foo/a/b/c.git",
                  "github.com",
                  "hoge/fuga/foo/a/b/c");
  }
}

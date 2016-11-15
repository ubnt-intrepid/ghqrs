use url::Url;
use error::GhqError;


#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
const KNOWN_HOSTS: &'static [(&'static str, usize)] = &[
    ("github.com", 2)
  , ("gist.github.com", 1)
  , ("bitbucket.org", 2)
  , ("gitlab.com", 2)
];


pub fn parse_token(s: &str) -> Result<(Url, String), GhqError> {
  let url = Url::parse(s).or_else(|_| make_remote_url(s))?;

  let host = url.host_str().ok_or("cannot retrieve host information")?;
  let path = url.path().trim_left_matches("/").trim_right_matches(".git");
  let path = format!("{}/{}", host, path);

  Ok((url, path))
}

fn make_remote_url(s: &str) -> Result<Url, GhqError> {
  let path: Vec<_> = s.split("/").collect();
  let path = match path.len() {
    0 => return Err("unsupported pattern to resolve remote URL").map_err(Into::into),
    1 => vec!["github.com", path[0], path[0]],
    2 => vec!["github.com", path[0], path[1]],
    _ => path,
  };

  Url::parse(&format!("{}://{}.git", "https", path.join("/"))).map_err(Into::into)
}


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

#[test]
fn long_path() {
  let url = make_remote_url("github.com/hoge/fuga/foo/a/b/c").unwrap();
  assert_eq!(url.as_str(), "https://github.com/hoge/fuga/foo/a/b/c.git");
}

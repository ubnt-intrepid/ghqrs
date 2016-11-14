use url::{Url, ParseError};


/// creates an instance of Url from str.
pub fn make_remote_url(s: &str) -> Result<Url, ParseError> {
  Url::parse(s).or_else(|_| make_remote_url_from_relative(s))
}

fn make_remote_url_from_relative(s: &str) -> Result<Url, ParseError> {
  let paths: Vec<_> = s.split("/").collect();

  let (host, user, repo) = match paths.len() {
    3 => (paths[0], paths[1], paths[2]),
    2 => ("github.com", paths[0], paths[1]),
    1 => ("github.com", paths[0], paths[0]),
    _ => panic!("'{}' is an unsupported pattern to resolve remote URL.", s),
  };

  Url::parse(&format!("https://{}/{}/{}.git", host, user, repo))
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

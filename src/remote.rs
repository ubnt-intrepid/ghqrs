extern crate url;

use url::Url;
use std::path::PathBuf;

pub struct RemoteRepository {
  protocol: String,
  base_url: String,
  user: String,
  project: String,
}

impl RemoteRepository {
  pub fn new(project: &str) -> RemoteRepository {
    let (protocol, base_url, user, repo);
    match url::Url::parse(project) {
      Ok(url) => {
        protocol = url.scheme().to_owned();
        base_url = url.host_str().unwrap().to_owned();

        let paths: Vec<_> = url.path_segments().unwrap().map(ToOwned::to_owned).collect();
        user = paths[0].clone();
        repo = paths[1].trim_right_matches(".git").to_owned();
      }
      Err(_) => {
        protocol = "https".to_owned();
        let paths: Vec<String> = project.split("/").map(ToOwned::to_owned).collect();
        if paths.len() == 3 {
          base_url = paths[0].clone();
          user = paths[1].clone();
          repo = paths[2].clone();
        } else if paths.len() == 2 {
          base_url = "github.com".to_owned();
          user = paths[0].clone();
          repo = paths[1].clone();
        } else {
          base_url = "github.com".to_owned();
          user = project.to_owned();
          repo = project.to_owned();
        }
      }
    };

    RemoteRepository {
      protocol: protocol,
      base_url: base_url,
      user: user,
      project: repo,
    }
  }

  pub fn url(&self) -> Url {
    Url::parse(&format!("{}://{}/{}/{}.git",
                        self.protocol,
                        self.base_url,
                        self.user,
                        self.project))
      .unwrap()
  }

  pub fn local_path(&self, root: &str) -> PathBuf {
    let mut dest = PathBuf::from(root);
    dest.push(&self.base_url);
    dest.push(&self.user);
    dest.push(&self.project);
    dest
  }
}

#[cfg(test)]
mod test {
  use super::RemoteRepository;

  #[test]
  fn user_project() {
    let repo = RemoteRepository::new("hoge/fuga");
    assert_eq!(repo.protocol, "https");
    assert_eq!(repo.base_url, "github.com");
    assert_eq!(repo.user, "hoge");
    assert_eq!(repo.project, "fuga");
  }

  #[test]
  fn domain_user_project() {
    let repo = RemoteRepository::new("github.com/hoge/fuga");
    assert_eq!(repo.protocol, "https");
    assert_eq!(repo.base_url, "github.com");
    assert_eq!(repo.user, "hoge");
    assert_eq!(repo.project, "fuga");
  }

  #[test]
  fn only_project_name() {
    let repo = RemoteRepository::new("github.com/hoge/fuga");
    assert_eq!(repo.protocol, "https");
    assert_eq!(repo.base_url, "github.com");
    assert_eq!(repo.user, "fuga");
    assert_eq!(repo.project, "fuga");
  }

  #[test]
  fn repository_url() {
    let repo = RemoteRepository::new("https://gitlab.com/funga-/pecopeco.git");
    assert_eq!(repo.protocol, "https");
    assert_eq!(repo.base_url, "gitlab.com");
    assert_eq!(repo.user, "funga-");
    assert_eq!(repo.project, "pecopeco");
  }
}

use std::collections::BTreeMap;
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, MAIN_SEPARATOR};
use toml;
use shellexpand;
use walkdir::WalkDir;

const CONFIG_CANDIDATES: &'static [&'static str] =
  &["~/.ghqconfig", "~/.config/ghq/config", ".ghqconfig"];

#[derive(RustcDecodable, Default)]
struct Config {
  roots: Vec<String>,
}

impl Config {
  pub fn load() -> Result<Config, io::Error> {
    let content = try!(read_file_if_exists(CONFIG_CANDIDATES))
      .expect("No configuration file found.");
    let mut config: Config = toml::decode_str(&content).unwrap();

    if config.roots.len() == 0 {
      let home_dir = env::home_dir().unwrap();
      let root_dir = home_dir.join(".ghq").to_str().map(ToOwned::to_owned).unwrap();
      config.roots = vec![root_dir];
    }

    for i in 0..(config.roots.len()) {
      config.roots[i] = shellexpand::full(&config.roots[i]).map(Cow::into_owned).unwrap();
    }

    Ok(config)
  }
}

pub struct Workspace {
  config: Config,
}

impl Workspace {
  pub fn init() -> Result<Workspace, io::Error> {
    let config = try!(Config::load());
    Ok(Workspace { config: config })
  }

  pub fn roots(&self) -> &[String] {
    self.config.roots.as_slice()
  }

  pub fn root(&self) -> Option<String> {
    self.config.roots.iter().next().cloned()
  }

  pub fn repositories(&self) -> BTreeMap<String, Vec<Repository>> {
    let mut dst = BTreeMap::new();

    for root in self.config.roots.iter() {
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
          let repo = Repository {
            vcs: vcs.unwrap(),
            root: root.to_owned(),
            path: path.replace("\\", "/"),
          };
          repos.push(repo);
        }
      }

      dst.insert(root.to_owned(), repos);
    }

    dst
  }
}


#[derive(Debug)]
pub struct Repository {
  vcs: String,
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


// Read the content of a file in `paths`
fn read_file_if_exists(paths: &[&str]) -> Result<Option<String>, io::Error> {
  for path in paths {
    // expand the candidate of path.
    let path = shellexpand::full(path).unwrap().into_owned();
    let path = Path::new(&path);

    if path.exists() && path.is_file() {
      let mut content = String::new();
      return File::open(path)
        .and_then(|ref mut file| file.read_to_string(&mut content))
        .and(Ok(Some(content)));
    }
  }
  Ok(None)
}

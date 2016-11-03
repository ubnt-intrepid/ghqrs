use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use toml;
use shellexpand;

const CONFIG_CANDIDATES: &'static [&'static str] =
  &["~/.ghqconfig", "~/.config/ghq/config", ".ghqconfig"];

#[derive(RustcDecodable, Default)]
pub struct Config {
  roots: Vec<String>,
}

impl Config {
  pub fn load() -> Result<Config, io::Error> {
    let content = try!(read_file_if_exists(CONFIG_CANDIDATES)).expect("No configuration file found.");
    let mut config: Config = toml::decode_str(&content).unwrap_or_default();

    if config.roots().len() == 0 {
      let home_dir = env::home_dir().unwrap();
      let root_dir = home_dir.join(".ghq").to_str().map(ToOwned::to_owned).unwrap();
      config.roots = vec![root_dir];
    }

    for i in 0..(config.roots().len()) {
      config.roots[i] = shellexpand::full(&config.roots[i]).map(Cow::into_owned).unwrap();
    }

    Ok(config)
  }

  pub fn roots(&self) -> &[String] {
    self.roots.as_slice()
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
      try!(File::open(path).and_then(|ref mut file| file.read_to_string(&mut content)));
      return Ok(Some(content));
    }
  }
  Ok(None)
}

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use toml;
use shellexpand;


#[cfg_attr(rustfmt, rustfmt_skip)]
const CANDIDATES: &'static [&'static str] = &[
    "~/.ghqconfig"
  , "~/.config/ghq/config"
  , ".ghqconfig"
];


#[derive(RustcDecodable)]
pub struct Config {
  pub roots: Vec<String>,
}

impl Default for Config {
  fn default() -> Config {
    Config { roots: vec!["~/.ghq".to_owned()] }
  }
}

impl Config {
  pub fn load() -> io::Result<Config> {
    let mut config: Config = read_file_if_exists(CANDIDATES)
      ?
      .and_then(|ref content| toml::decode_str(content))
      .unwrap_or_default();

    for i in 0..(config.roots.len()) {
      config.roots[i] = shellexpand::full(&config.roots[i]).map(Cow::into_owned).unwrap();
    }

    Ok(config)
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

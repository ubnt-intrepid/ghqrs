use std::borrow::Cow;
use std::env::VarError;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use toml;
use shellexpand::{self, LookupError};


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
    let content = read_file_if_exists(CANDIDATES)?;
    let mut config: Config = content.and_then(|s| toml::decode_str(&s)).unwrap_or_default();

    for i in 0..(config.roots.len()) {
      config.roots[i] = expand_full(&config.roots[i]).unwrap();
    }

    Ok(config)
  }
}


fn expand_full(s: &str) -> Result<String, LookupError<VarError>> {
  shellexpand::full(s).map(Cow::into_owned)
}


// Read the content of a file in `paths`
fn read_file_if_exists(paths: &[&str]) -> io::Result<Option<String>> {
  let path = paths.iter()
    .map(|s| expand_full(s).unwrap())
    .map(PathBuf::from)
    .filter(|ref p| p.is_file())
    .next();

  match path {
    Some(path) => {
      let mut content = String::new();
      File::open(path)?.read_to_string(&mut content)?;
      Ok(Some(content))
    }
    None => Ok(None),
  }
}

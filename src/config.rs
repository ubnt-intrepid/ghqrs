use std::io;
use std::env;
use std::borrow::Cow;
use util;
use toml;
use shellexpand;


const CONFIG_CANDIDATES: &'static [&'static str] =
  &["~/.ghqconfig", "~/.config/ghq/config", ".ghqconfig"];


#[derive(RustcDecodable, Default)]
pub struct Config {
  pub roots: Vec<String>,
}

impl Config {
  pub fn load() -> Result<Config, io::Error> {
    let content = try!(util::read_file_if_exists(CONFIG_CANDIDATES))
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

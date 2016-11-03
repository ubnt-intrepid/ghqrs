use std::env;
use std::fs::File;
use std::io::Read;
use toml;
use shellexpand;

pub fn get_roots() -> Vec<String> {
  let mut file = File::open(".ghqconfig").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  let config = toml::Parser::new(&content).parse().expect("failed to parse .ghqconfig");

  let roots;
  if let Some(r) = config.get("root") {
    match *r {
      toml::Value::String(ref s) => {
        roots = vec![shellexpand::full(s).unwrap().into_owned()];
      }
      toml::Value::Array(ref a) => {
        roots = a.iter()
          .map(|a| shellexpand::full(a.as_str().unwrap()).unwrap().into_owned())
          .collect::<Vec<_>>();
      }
      _ => panic!("The type of 'root' is invalid"),
    }
  } else {
    roots = vec![format!("{}", env::home_dir().unwrap().join(".ghq").display())];
  }

  roots
}

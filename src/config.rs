use std::env;
use std::process::Command;

pub fn get_roots() -> Vec<String> {
  let mut roots;

  let env_root = env::var("GHQ_ROOT").unwrap_or("".to_owned());
  if env_root == "" {
    roots = vec![get_config("ghq.root")];
  } else {
    roots = env_root.split(":").map(|s| s.to_owned()).collect();
  }

  if roots.len() == 0 {
    roots.push(format!("{}", env::home_dir().unwrap().join(".ghq").display()));
  }

  assert!(roots.len() >= 1);

  roots
}

pub fn get_config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8_lossy(&output.stdout[0..len - 1]).into_owned()
}

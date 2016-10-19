use std::env;
use std::process::Command;

pub fn get_roots() -> Vec<String> {
  let mut root = get_config("ghq.root").trim().to_owned();
  if root == "" {
    root = format!("{}", env::home_dir().unwrap().join(".ghq").display());
  }

  vec![root]
}

pub fn get_config(key: &str) -> String {
  let output = Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  let len = output.stdout.len();
  String::from_utf8_lossy(&output.stdout[0..len - 1]).into_owned()
}

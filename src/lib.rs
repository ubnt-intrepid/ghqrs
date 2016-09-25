pub fn command_list(exact: bool, fullpath: bool, unique: bool) -> i32 {
  println!("list: Not implemented, {}, {}, {}", exact, fullpath, unique);
  0
}

fn git_config(key: &str) -> String {
  let output = std::process::Command::new("git")
    .args(&["config", "--path", "--null", "--get-all", key])
    .output()
    .expect("failed to execute git");
  String::from_utf8(output.stdout).unwrap()
}

fn get_local_repos_roots() -> Vec<String> {
  let mut local_repo_roots;

  let env_root: String = std::env::var("GHQ_ROOT").unwrap_or("".to_owned());
  if env_root == "" {
    local_repo_roots = vec![git_config("ghq.root")];
  } else {
    local_repo_roots = env_root.split(":").map(|s| s.to_owned()).collect();
  }

  if local_repo_roots.len() == 0 {
    let mut ghq_path = std::env::home_dir().unwrap();
    ghq_path.push(".ghq");
    local_repo_roots.push(format!("{}", ghq_path.display()));
  }

  assert!(local_repo_roots.len() >= 1);

  local_repo_roots
}

pub fn command_root(all: bool) -> i32 {
  let roots = get_local_repos_roots();
  if all {
    for root in roots {
      println!("{}", root);
    }
  } else {
    println!("{}", roots[0]);
  }
  0
}

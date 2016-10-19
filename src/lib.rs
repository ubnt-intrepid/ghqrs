extern crate walkdir;
extern crate regex;
extern crate url;

mod config;
mod local;
mod remote;
mod util;
mod vcs;

use local::Repository;
use remote::RemoteRepository;


pub fn command_get(projects: Vec<String>, pull: bool, depth: Option<i32>) -> i32 {
  for project in projects {
    let url = remote::make_remote_url(&project).unwrap();
    let repo = RemoteRepository::new(url).unwrap();
    repo.clone_or_pull(&config::get_roots()[0], pull, depth).unwrap();
  }
  0
}

pub fn command_list(exact: bool, format: &str, query: Option<String>) -> i32 {
  let filter: Box<Fn(&Repository) -> bool>;
  if let Some(query) = query {
    if exact {
      filter = Box::new(move |repo: &Repository| repo.project_name() == query);
    } else {
      filter = Box::new(move |repo: &Repository| repo.contains(&query));
    }
  } else {
    filter = Box::new(|_| true);
  }

  for repo in local::get_local_repositories().into_iter().filter(|ref repo| filter(repo)) {
    let path = match format {
      "full" => repo.absolute_path(),
      "unique" => repo.unique_path(),
      _ => repo.relative_path(),
    };
    println!("{}", path);
  }

  0
}

pub fn command_root(all: bool) -> i32 {
  let roots = config::get_roots();
  if all {
    for root in roots {
      println!("{}", root);
    }
  } else {
    println!("{}", roots[0]);
  }
  0
}

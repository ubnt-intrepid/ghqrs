extern crate walkdir;
extern crate regex;
extern crate url;

mod config;
mod repository;
mod util;
mod vcs;

use repository::{LocalRepository, RemoteRepository};


pub fn command_get(projects: Vec<String>, pull: bool, depth: Option<i32>) -> i32 {
  for project in projects {
    let url = repository::make_remote_url(&project).unwrap();
    let repo = RemoteRepository::new(url).unwrap();
    repo.clone_or_pull(&config::get_roots()[0], pull, depth).unwrap();
  }
  0
}

pub fn command_list(exact: bool, format: &str, query: Option<String>) -> i32 {
  let filter: Box<Fn(&LocalRepository) -> bool>;
  if let Some(query) = query {
    if exact {
      filter = Box::new(move |repo: &LocalRepository| repo.project_name() == query);
    } else {
      filter = Box::new(move |repo: &LocalRepository| repo.contains(&query));
    }
  } else {
    filter = Box::new(|_| true);
  }

  for (_, repos) in repository::get_local_repositories(|ref repo| filter(repo)) {
    for repo in repos {
      let path = match format {
        "full" => repo.absolute_path(),
        "unique" => repo.unique_path(),
        _ => repo.relative_path(),
      };
      println!("{}", path);
    }
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

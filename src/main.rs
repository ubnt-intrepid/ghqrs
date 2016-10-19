extern crate clap;
extern crate walkdir;
extern crate regex;
extern crate url;

mod cli;
mod config;
mod repository;
mod util;
mod vcs;

use repository::{LocalRepository, RemoteRepository};

fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = match matches.subcommand() {
    ("get", Some(m)) => {
      let projects = m.values_of("project").unwrap().map(ToOwned::to_owned).collect();
      let pull = m.is_present("pull");
      let depth = m.value_of("depth").map(|ref d| d.parse::<i32>().unwrap());
      command_get(projects, pull, depth)
    }
    ("list", Some(m)) => {
      let exact = m.is_present("exact");
      let format = m.value_of("format").unwrap_or("default");
      let query = m.value_of("query").map(ToOwned::to_owned);
      command_list(exact, format, query)
    }
    ("root", Some(m)) => {
      let all = m.is_present("all");
      command_root(all)
    }
    (_, _) => unreachable!(),
  };

  std::process::exit(exitcode);
}


fn command_get(projects: Vec<String>, pull: bool, depth: Option<i32>) -> i32 {
  for project in projects {
    let url = repository::make_remote_url(&project).unwrap();
    let repo = RemoteRepository::new(url).unwrap();
    repo.clone_or_pull(&config::get_roots()[0], pull, depth).unwrap();
  }
  0
}

fn command_list(exact: bool, format: &str, query: Option<String>) -> i32 {
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

fn command_root(all: bool) -> i32 {
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

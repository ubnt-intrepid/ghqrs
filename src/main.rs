extern crate clap;
extern crate walkdir;
extern crate regex;
extern crate url;

mod cli;
mod config;
mod repository;
mod util;
mod vcs;

use repository::RemoteRepository;

// output format
enum ListFormat {
  // relative path from host directory
  // e.g. github.com/hoge/fuga
  Default,

  // absolute path
  // e.g. /home/hoge/github.com/hoge/fuga or C:\Users\hoge\github.com\hoge\fuga
  FullPath,

  // only project name
  // e.g. fuga
  Unique,
}

impl<'a> From<&'a str> for ListFormat {
  fn from(s: &str) -> ListFormat {
    match s {
      "full" => ListFormat::FullPath,
      "unique" => ListFormat::Unique,
      _ => ListFormat::Default,
    }
  }
}


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
      let format = m.value_of("format").unwrap_or("default").into();
      command_list(format)
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

fn command_list(format: ListFormat) -> i32 {
  for (_, repos) in repository::get_local_repositories() {
    for repo in repos {
      let path = match format {
        ListFormat::Default => repo.relative_path(),
        ListFormat::FullPath => repo.absolute_path(),
        ListFormat::Unique => repo.unique_path(),
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

extern crate clap;
extern crate walkdir;
extern crate regex;
extern crate url;
extern crate shellexpand;
extern crate toml;

mod config;
mod repository;
mod util;
mod vcs;

use clap::{Arg, App, AppSettings, SubCommand};
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
  let matches = cli().get_matches();

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

fn cli() -> App<'static, 'static> {
  App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("get")
      .about("Clone or sync with remote repository")
      .arg(Arg::with_name("project")
        .multiple(true)
        .required(true)
        .help("repository name or URL"))
      .arg(Arg::with_name("pull")
        .long("pull")
        .help("Pull active branch if the local repository has already existed"))
      .arg(Arg::with_name("depth")
        .long("depth")
        .takes_value(true)
        .help("The number of commit history (i.e. do shallow clone)")))
    .subcommand(SubCommand::with_name("list")
      .about("List locally cloned repositories")
      .arg(Arg::with_name("format")
        .short("f")
        .long("format")
        .takes_value(true)
        .help("Output format of paths")))
    .subcommand(SubCommand::with_name("root")
      .about("Show repositories's root")
      .arg(Arg::with_name("all")
        .short("a")
        .long("all")
        .help("Show all roots")))
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

extern crate clap;
extern crate walkdir;
extern crate regex;
extern crate url;
extern crate shellexpand;
extern crate toml;
extern crate rustc_serialize;

mod remote;
mod util;
mod vcs;
mod workspace;

use clap::{Arg, App, AppSettings, SubCommand};
use workspace::Workspace;

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
    ("clone", Some(m)) => {
      let queries = m.values_of("query").unwrap().map(ToOwned::to_owned).collect();
      command_clone(queries)
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
    .subcommand(SubCommand::with_name("clone")
      .about("Clone remote repository into your working directory")
      .arg(Arg::with_name("query")
        .multiple(true)
        .required(true)
        .help("repository name or URL")))
    .subcommand(SubCommand::with_name("list")
      .about("List local repositories into the working directories")
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


fn command_clone(queries: Vec<String>) -> i32 {
  let workspace = Workspace::init()
    .unwrap_or_else(|e| panic!("failed to initialize workspace: {:?}", e));
  let root = workspace.root().expect("cannot get the destination directory of targets");

  for query in queries {
    let url = remote::make_remote_url(&query).unwrap();
    let repo = remote::RemoteRepository::new(url).unwrap();
    repo.clone(&root, None).unwrap();
  }
  0
}

fn command_list(format: ListFormat) -> i32 {
  let workspace = Workspace::init()
    .unwrap_or_else(|e| panic!("failed to initialize workspace: {:?}", e));
  for (_, repos) in workspace.repositories() {
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
  let workspace = Workspace::init()
    .unwrap_or_else(|e| panic!("failed to initialize workspace: {:?}", e));
  if all {
    for root in workspace.roots() {
      println!("{}", root);
    }
  } else {
    println!("{}", workspace.root().unwrap());
  }
  0
}

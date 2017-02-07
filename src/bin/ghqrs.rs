extern crate ghq;
extern crate clap;

use std::io::{self, Write};
use clap::{Arg, App, AppSettings, SubCommand};
use ghq::config::Config;
use ghq::workspace::Workspace;
use ghq::error::GhqError;

fn main() {
  match _main() {
    Ok(exitcode) => std::process::exit(exitcode),
    Err(err) => writeln!(&mut io::stderr(), "IO Error: {}", err.to_string()).unwrap(),
  }
}

fn _main() -> Result<i32, GhqError> {
  let config = Config::load()?;
  let workspace = Workspace::new(config);

  let matches = cli().get_matches();
  match matches.subcommand() {
    ("clone", Some(m)) => {
      let queries = m.values_of("query").unwrap();
      for ref s in queries {
        workspace.clone_from(s)?;
      }
    }
    ("list", Some(_)) => {
      workspace.map_repo(|repo, root| {
        let repo_path = repo.local_path(root).to_string_lossy().replace("\\", "/");
        println!("{}", repo_path);
      });
    }
    ("root", Some(m)) => {
      if m.is_present("all") {
        for root in workspace.roots() {
          println!("{}", root.display());
        }
      } else {
        if let Some(root) = workspace.default_root() {
          println!("{}", root.display());
        }
      }
    }
    (_, _) => unreachable!(),
  };

  Ok(0)
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
        .help("repository name or URL"))
      .arg(Arg::with_name("root")
        .long("root")
        .help("root directory of cloned repository")))
    .subcommand(SubCommand::with_name("list")
      .about("List local repositories into the working directories"))
    .subcommand(SubCommand::with_name("root")
      .about("Show repositories's root")
      .arg(Arg::with_name("all")
        .short("a")
        .long("all")
        .help("Show all roots")))
}

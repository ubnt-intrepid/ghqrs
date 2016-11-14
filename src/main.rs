extern crate clap;
extern crate regex;
extern crate rustc_serialize;
extern crate shellexpand;
extern crate toml;
extern crate url;
extern crate walkdir;

mod config;
mod remote;
mod util;
mod vcs;
mod workspace;

use std::io::{self, Write};
use clap::{Arg, App, AppSettings, SubCommand};
use config::Config;
use workspace::Workspace;

fn main() {
  match _main() {
    Ok(exitcode) => std::process::exit(exitcode),
    Err(err) => writeln!(&mut io::stderr(), "IO Error: {}", err.to_string()).unwrap(),
  }
}

fn _main() -> io::Result<i32> {
  let config = try!(Config::load());
  let workspace = Workspace::new(config);

  let matches = cli().get_matches();
  match matches.subcommand() {
    ("clone", Some(m)) => {
      let queries = m.values_of("query").unwrap();
      for ref query in queries {
        workspace.clone_repository(query);
      }
    }
    ("list", Some(m)) => {
      let format = m.value_of("format").unwrap_or("default").into();
      workspace.show_repositories(format);
    }
    ("root", Some(m)) => {
      let all = m.is_present("all");
      workspace.show_roots(all);
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

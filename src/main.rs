extern crate ghqrs;
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
  let matches = App::new("ghqrs")
    .about("Remote management")
    .version("0.0.1")
    .author("Yusuke Sasaki <yusuke.sasaki.nuem@gmail.com>")
    .subcommand(SubCommand::with_name("get")
      .about("clone/sync")
      .arg(Arg::with_name("project").help("repository name")))
    .subcommand(SubCommand::with_name("list")
      .about("List locally cloned repositories")
      .arg(Arg::with_name("exact")
        .short("e")
        .long("exact")
        .help("Perform an exact match"))
      .arg(Arg::with_name("fullpath")
        .short("p")
        .long("full-path")
        .help("print full paths"))
      .arg(Arg::with_name("unique")
        .short("u")
        .long("unique")
        .help("Print unique subpaths"))
      .arg(Arg::with_name("query")))
    .subcommand(SubCommand::with_name("root")
      .about("Show repositories's root")
      .arg(Arg::with_name("all")
        .short("a")
        .long("all")
        .help("Show all roots")))
    .get_matches();

  let exitcode = match matches.subcommand_name() {
    Some(ref s) => {
      let ref matches = matches.subcommand_matches(s).unwrap();
      match *s {
        "get" => {
          let project = match matches.value_of("project") {
            Some(project) => project.to_owned(),
            None => panic!("project not found"),
          };
          ghqrs::command_get(project)
        }
        "list" => {
          let exact = matches.is_present("exact");
          let fullpath = matches.is_present("fullpath");
          let unique = matches.is_present("unique");
          let query = matches.value_of("query").map(ToOwned::to_owned);
          ghqrs::command_list(exact, fullpath, unique, query)
        }
        "root" => {
          let all = matches.is_present("all");
          ghqrs::command_root(all)
        }
        _ => panic!("invalid subcommand: {}", s),
      }
    }
    None => panic!("Invalid subcommand"),
  };
  std::process::exit(exitcode);
}

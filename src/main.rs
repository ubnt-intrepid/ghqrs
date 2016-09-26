extern crate ghqrs;
extern crate clap;

mod cli;

fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = if let Some(ref s) = matches.subcommand_name() {
    let ref matches = matches.subcommand_matches(s).unwrap();
    match *s {
      "get" => {
        let projects = matches.values_of("project").unwrap().map(ToOwned::to_owned).collect();
        let skip_pull = matches.is_present("skip-pull");
        let shallow = matches.is_present("shallow");
        ghqrs::command_get(projects, skip_pull, shallow)
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
      _ => unreachable!(),
    }
  } else {
    unreachable!()
  };
  std::process::exit(exitcode);
}

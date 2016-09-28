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
        let pull = matches.is_present("pull");
        let depth =
          matches.value_of("depth").map(ToOwned::to_owned).map(|d| d.parse::<i32>().unwrap());
        ghqrs::command_get(projects, pull, depth)
      }
      "list" => {
        let exact = matches.is_present("exact");
        let format = matches.value_of("format").unwrap_or("default");
        let query = matches.value_of("query").map(ToOwned::to_owned);
        ghqrs::command_list(exact, format, query)
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

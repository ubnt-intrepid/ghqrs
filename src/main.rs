extern crate ghqrs;
extern crate clap;

mod cli;

fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = match matches.subcommand() {
    ("get", Some(m)) => {
      let projects = m.values_of("project").unwrap().map(ToOwned::to_owned).collect();
      let pull = m.is_present("pull");
      let depth = m.value_of("depth").map(|ref d| d.parse::<i32>().unwrap());
      ghqrs::command_get(projects, pull, depth)
    }
    ("list", Some(m)) => {
      let exact = m.is_present("exact");
      let format = m.value_of("format").unwrap_or("default");
      let query = m.value_of("query").map(ToOwned::to_owned);
      ghqrs::command_list(exact, format, query)
    }
    ("root", Some(m)) => {
      let all = m.is_present("all");
      ghqrs::command_root(all)
    }
    (_, _) => unreachable!(),
  };

  std::process::exit(exitcode);
}

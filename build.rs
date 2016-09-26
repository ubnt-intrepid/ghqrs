extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
  let mut app = build_cli();
  app.gen_completions("ghqrs", Shell::Bash, env!("OUT_DIR"));
}

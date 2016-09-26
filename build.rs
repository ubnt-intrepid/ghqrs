extern crate clap;

use std::path::Path;
use std::fs::OpenOptions;
use clap::Shell;

include!("src/cli.rs");

fn main() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(Path::new(env!("CARGO_MANIFEST_DIR")).join("_ghqrs").to_str().unwrap())
    .unwrap();

  build_cli().gen_completions_to("ghqrs", Shell::Bash, &mut file);
}

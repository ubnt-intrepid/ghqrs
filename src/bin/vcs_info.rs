extern crate ghq;

use std::env;
use std::io::{self, Write};
use ghq::vcs;

fn main() {
  // ensure that all outputs are English.
  env::set_var("LANGUAGE", "en_US.UTF-8");
  env::set_var("LANG", "en_US.UTF-8");

  let fallback = env::args().skip(1).next().map(|s| s.trim() == "--fallback").unwrap_or(false);

  match vcs::current_status(&env::current_dir().unwrap()) {
    Ok(Some(s)) => io::stdout().write_all(s.prompt(fallback).as_bytes()).unwrap(),
    _ => (),
  }
}

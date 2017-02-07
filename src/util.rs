use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::{Command, Stdio};

pub trait SplitEOL {
  fn split_eol(&self) -> Vec<String>;
}

impl SplitEOL for String {
  fn split_eol(&self) -> Vec<String> {
    if self.trim() == "" {
      Vec::new()
    } else {
      self.split("\n").map(ToOwned::to_owned).collect()
    }
  }
}

pub fn communicate(name: &str, args: &[&str]) -> Result<(String, String, i32), io::Error> {
  use std::process::*;

  let output = Command::new(name).args(args)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()?;
  let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
  let stderr = String::from_utf8_lossy(&output.stdout).into_owned();
  let status = output.status
    .code()
    .ok_or(io::Error::new(io::ErrorKind::Other,
                          "The process was terminated by a signal"))?;

  Ok((stdout, stderr, status))
}

pub fn get_lines(name: &str, args: &[&str]) -> Result<Vec<String>, io::Error> {
  communicate(name, args).map(|(stdout, _, _)| stdout.split_eol())
}

pub fn read_content<P: AsRef<Path>>(path: P) -> io::Result<String> {
  let mut buf = String::new();
  File::open(path)
    .and_then(|mut f| f.read_to_string(&mut buf))
    .and(Ok(buf.trim().to_owned()))
}

pub fn wait_exec(cmd: &str, args: &[&str], curr_dir: Option<&Path>) -> Result<i32, io::Error> {
  let mut command = Command::new(cmd);
  command.args(args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());
  if let Some(curr_dir) = curr_dir {
    command.current_dir(curr_dir);
  }

  let mut child = command.spawn()?;
  child.wait()
    .and_then(|st| st.code().ok_or(io::Error::new(io::ErrorKind::Other, "")))
}

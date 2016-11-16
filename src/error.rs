use std::io;
use std::path::StripPrefixError;
use url::ParseError;


#[derive(Debug)]
pub enum GhqError {
  IO(io::Error),
  StripPrefix(StripPrefixError),
  UrlParse(ParseError),
  Other(&'static str),
}

impl GhqError {
  pub fn to_string(&self) -> String {
    match *self {
      GhqError::IO(ref err) => err.to_string(),
      GhqError::StripPrefix(ref err) => err.to_string(),
      GhqError::UrlParse(ref err) => err.to_string(),
      GhqError::Other(ref err) => err.to_string(),
    }
  }
}

impl From<io::Error> for GhqError {
  fn from(err: io::Error) -> GhqError {
    GhqError::IO(err)
  }
}

impl From<StripPrefixError> for GhqError {
  fn from(err: StripPrefixError) -> GhqError {
    GhqError::StripPrefix(err)
  }
}

impl From<ParseError> for GhqError {
  fn from(err: ParseError) -> GhqError {
    GhqError::UrlParse(err)
  }
}

impl From<&'static str> for GhqError {
  fn from(err: &'static str) -> GhqError {
    GhqError::Other(err)
  }
}

use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
}

impl Error {
  pub fn new(kind: ErrorKind) -> Self {
    Error { kind }
  }

  pub fn kind(&self) -> &ErrorKind {
    &self.kind
  }
}

/**
 * Breakdown #1:
 *
 * - `enums` fields are by default public, if the enum is public
 *    unlike `structs`.
 */
#[derive(Debug)]
pub enum ErrorKind {
  NoArgs,
  TooMany(usize),
}

impl ErrorKind {
  pub fn as_str(&self) -> &str {
    match *self {
      ErrorKind::NoArgs => "no arguments passed",
      ErrorKind::TooMany(count) => "too many arguments",
    }
  }
}

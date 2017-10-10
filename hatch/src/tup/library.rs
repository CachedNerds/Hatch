use std::fmt;

#[derive(Debug)]
pub enum LibraryKind {
  Shared,
  Static,
}

impl fmt::Display for LibraryKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      &Shared => write!(f, "shared"),
      &Static => write!(f, "static"),
    }
  }
}

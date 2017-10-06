use std::{ io, fmt };

extern crate clap;

#[derive(Debug)]
pub enum Error {
  IOError(io::Error),
  ClapError(clap::Error),
  E(&'static str),
  NullError,
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::IOError(e)
  }
}

impl From<clap::Error> for Error {
  fn from(e: clap::Error) -> Self {
    Error::ClapError(e)
  }
}

impl From<&'static str> for Error {
  fn from(e: &'static str) -> Self {
    Error::E(e)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::IOError(ref e) => e.fmt(f),
      &Error::ClapError(ref e) => e.fmt(f),
      &Error::E(ref e) => write!(f, "{}", e),
      &Error::NullError => write!(f, "{}", "An error occured"),
    }
  }
}

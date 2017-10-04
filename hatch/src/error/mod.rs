use std::{ io, fmt };

extern crate clap;

#[derive(Debug)]
pub enum Error {
  IOError(io::Error),
  ClapError(clap::Error),
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

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::IOError(ref e) => write!(f, "{}", e),
      &Error::ClapError(ref e) => write!(f, "{}", e),
      &Error::NullError => write!(f, "{}", "An error occured"),
    }
  }
}

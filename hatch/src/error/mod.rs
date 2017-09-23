use std::io;

extern crate clap;

#[derive(Debug)]
pub enum Error {
  IOError(io::Error),
  ClapError(clap::Error),
  NullError,
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Error::IOError(error)
  }
}

impl From<clap::Error> for Error {
  fn from(error: clap::Error) -> Self {
    Error::ClapError(error)
  }
}

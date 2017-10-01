use std::io;

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

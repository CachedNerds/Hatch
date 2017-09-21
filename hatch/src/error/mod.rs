use std::io;

extern crate clap;

#[derive(Debug)]
pub enum ErrorT {
  IOError(io::Error),
  ClapError(clap::Error),
  NullError,
}

impl From<io::Error> for ErrorT {
  fn from(error: io::Error) -> Self {
    ErrorT::IOError(error)
  }
}

impl From<clap::Error> for ErrorT {
  fn from(error: clap::Error) -> Self {
    ErrorT::ClapError(error)
  }
}

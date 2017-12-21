use std::error::Error;
use std::fmt;
use super::HatchError;

#[derive(Debug)]
pub struct NullError;

impl From<NullError> for HatchError {
  fn from(e: NullError) -> Self {
    HatchError::Null(e)
  }
}

impl fmt::Display for NullError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for NullError {
  fn description(&self) -> &str {
    ""
  }
  
  fn cause(&self) -> Option<&Error> {
    None
  }
}

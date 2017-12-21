use std::error::Error;
use std::fmt;
use super::HatchError;

#[derive(Debug)]
pub struct MissingNameError;

impl From<MissingNameError> for HatchError {
  fn from(e: MissingNameError) -> Self {
    HatchError::MissingName(e)
  }
}

impl fmt::Display for MissingNameError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for MissingNameError {
  fn description(&self) -> &str {
    "No name field found in Hatch.yml"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

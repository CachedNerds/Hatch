use std::error::Error;
use std::fmt;
use super::HatchError;

#[derive(Debug)]
pub struct MissingVersionError;

impl From<MissingVersionError> for HatchError {
  fn from(e: MissingVersionError) -> Self {
    HatchError::MissingVersion(e)
  }
}

impl fmt::Display for MissingVersionError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for MissingVersionError {
  fn description(&self) -> &str {
    "No version field found in Hatch.yml"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

use std::error::Error;
use std::fmt;
use super::HatchError;

#[derive(Debug)]
pub struct MissingBuildError;

impl From<MissingBuildError> for HatchError {
  fn from(e: MissingBuildError) -> Self {
    HatchError::MissingBuild(e)
  }
}

impl fmt::Display for MissingBuildError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for MissingBuildError {
  fn description(&self) -> &str {
    "No build field found in Hatch.yml"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

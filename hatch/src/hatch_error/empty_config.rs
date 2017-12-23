use std::error::Error;
use std::fmt;
use super::HatchError;

#[derive(Debug)]
pub struct EmptyConfigError;

impl From<EmptyConfigError> for HatchError {
  fn from(e: EmptyConfigError) -> Self {
    HatchError::EmptyConfig(e)
  }
}

impl fmt::Display for EmptyConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for EmptyConfigError {
  fn description(&self) -> &str {
    "No content in Hatch.yml"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}


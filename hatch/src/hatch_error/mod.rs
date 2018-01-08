use std::fmt;
use std::error::Error;

use std::io;
use yaml_rust::scanner;
use git2;

pub mod null;
pub mod empty_config;
pub mod missing_build;
pub mod missing_name;
pub mod missing_version;

pub use self::null::NullError;
pub use self::empty_config::EmptyConfigError;
pub use self::missing_name::MissingNameError;
pub use self::missing_build::MissingBuildError;
pub use self::missing_version::MissingVersionError;

#[derive(Debug)]
pub enum HatchError {
  Git(git2::Error),
  Io(io::Error),
  Parsing(scanner::ScanError),
  Null(NullError),
  EmptyConfig(EmptyConfigError),
  MissingName(MissingNameError),
  MissingBuild(MissingBuildError),
  MissingVersion(MissingVersionError),
}

impl Error for HatchError {
  fn description(&self) -> &str {
    match *self {
      _ => self.description()
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      _ => self.cause()
    }
  }
}

impl fmt::Display for HatchError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      HatchError::Git(ref e) => write!(f, "Git error: {}", e),
      HatchError::Io(ref e) => write!(f, "IO error: {}", e),
      HatchError::Parsing(ref e) => write!(f, "Parsing error: {}", e),
      HatchError::Null(ref e) => write!(f, "{}", e),
      HatchError::EmptyConfig(ref e) => write!(f, "Config error: {}", e),
      HatchError::MissingName(ref e) => write!(f, "Config error: {}", e),
      HatchError::MissingBuild(ref e) => write!(f, "Config error: {}", e),
      HatchError::MissingVersion(ref e) => write!(f, "Config error: {}", e),
    }
  }
}

impl From<scanner::ScanError> for HatchError {
  fn from(e: scanner::ScanError) -> Self {
    HatchError::Parsing(e)
  }
}

impl From<io::Error> for HatchError {
  fn from(e: io::Error) -> Self {
    HatchError::Io(e)
  }
}

impl From<git2::Error> for HatchError {
  fn from(e: git2::Error) -> Self {
    HatchError::from(e)
  }
}

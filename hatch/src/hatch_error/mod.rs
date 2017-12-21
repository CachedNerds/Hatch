use std::fmt;
use std::error::Error;

use std::io;
use yaml_rust::scanner;

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
      HatchError::Io(ref e) => e.description(),
      HatchError::Parsing(ref e) => e.description(),
      HatchError::Null(ref e) => e.description(),
      HatchError::EmptyConfig(ref e) => e.description(),
      HatchError::MissingName(ref e) => e.description(),
      HatchError::MissingBuild(ref e) => e.description(),
      HatchError::MissingVersion(ref e) => e.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      HatchError::Io(ref e) => e.cause(),
      HatchError::Parsing(ref e) => e.cause(),
      HatchError::Null(ref e) => e.cause(),
      HatchError::EmptyConfig(ref e) => e.cause(),
      HatchError::MissingName(ref e) => e.cause(),
      HatchError::MissingBuild(ref e) => e.cause(),
      HatchError::MissingVersion(ref e) => e.cause(),
    }
  }
}

impl fmt::Display for HatchError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
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

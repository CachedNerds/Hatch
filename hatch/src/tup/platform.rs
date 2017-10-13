use std::fmt;

#[derive(Debug)]
pub enum PlatformKind {
  Linux,
  Darwin,
  Windows,
}

impl PlatformKind {
  pub fn from_str(string: &str) -> PlatformKind {
    match string {
      "Linux" => PlatformKind::Linux,
      "Darwin" => PlatformKind::Darwin,
      _  => PlatformKind::Windows,
    }
  }
}

impl fmt::Display for PlatformKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      &Linux => write!(f, "Linux"),
      &Darwin => write!(f, "Darwin"),
      &Windows => write!(f, "Windows"),
    }
  }
}

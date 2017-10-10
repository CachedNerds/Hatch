use std::fmt;

#[derive(Debug)]
pub enum PlatformKind {
  Linux,
  Mac,
  Windows,
}

impl PlatformKind {
  pub fn from_str(string: &str) -> PlatformKind {
    match string {
      "linux" => PlatformKind::Linux,
      "mac" => PlatformKind::Mac,
      _  => PlatformKind::Windows,
    }
  }
}

impl fmt::Display for PlatformKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      &Linux => write!(f, "linux"),
      &OSX => write!(f, "osx"),
      &Windows => write!(f, "dos"),
    }
  }
}

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

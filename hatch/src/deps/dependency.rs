use std::path::PathBuf;
use std::ffi::OsString;
use std::fmt;

#[derive(Debug)]
pub struct Dependency {
  url: String,
  name: String,
}

impl Dependency {
  pub fn new(url: String) -> Dependency {
    Dependency {
      name: PathBuf::from(OsString::from(url.clone()))
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .into_owned()
        .as_str()
        .replace(".git", ""),
      url,
    }
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  pub fn url(&self) -> &str {
    self.url.as_ref()
  }
}

impl fmt::Display for Dependency {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.url)
  }
}
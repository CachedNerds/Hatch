use std::path::PathBuf;
use std::ffi::OsStr;
use std::fmt;

#[derive(Debug)]
pub struct Dependency {
  url: String,
  name: String,
}

impl Dependency {
  pub fn new(url: String) -> Dependency {
    Dependency {
      name: (|u: &OsStr| -> String {
        String::from(u.to_string_lossy())[..].replace(".git", "")
      })(PathBuf::from(url.clone()).iter().last().unwrap()),
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

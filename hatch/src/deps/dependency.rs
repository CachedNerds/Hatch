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
      name: Dependency::extract_name(url.clone()),
      url,
    }
  }

  fn extract_name(url: String) -> String {
    let url_path = PathBuf::from(url);
    let last_element = url_path.iter().last().unwrap();
    (last_element.as_ref() as &OsStr).to_string_lossy()[..].replace(".git", "")
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

use std::fmt;
use std::path::PathBuf;
use std::ffi::OsString;


#[derive(Debug)]
pub struct Project {
  name: String,
  kind: ProjectKind,
  version: String,
  deps: Vec<Dependency>,
  path: PathBuf
}

impl Project {
  pub fn new(name: String,
             kind: ProjectKind,
             version: String,
             deps: Vec<Dependency>,
             path: PathBuf) -> Project
  {
    Project { name, kind, version, deps, path }
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  pub fn kind(&self) -> &ProjectKind {
    self.kind.as_ref()
  }
  
  pub fn version(&self) -> &str {
    self.version.as_ref()
  }

  pub fn deps(&self) -> &Vec<Dependency> {
    self.deps.as_ref()
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }
}

#[derive(Debug)]
pub enum LibraryKind { Shared, Static }

#[derive(Debug)]
pub enum ProjectKind { Binary, Library(LibraryKind) }

impl AsRef<ProjectKind> for ProjectKind {
  fn as_ref(&self) -> &ProjectKind {
    &self
  }
}

impl fmt::Display for ProjectKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ProjectKind::Library(LibraryKind::Shared) => write!(f, "shared-lib"),
      ProjectKind::Library(LibraryKind::Static) => write!(f, "static-lib"),
      ProjectKind::Binary => write!(f, "binary"),
    }
  }
}

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

  pub fn as_pair(&self) -> (String, String) {
    (self.url.to_owned(), self.name.to_owned())
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

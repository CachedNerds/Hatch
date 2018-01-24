use std::fmt;
use std::path::PathBuf;

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
  name: String,
  url: String,
}

impl Dependency {
  pub fn new(name: String, url: String) -> Dependency {
    Dependency { name, url }
  }

  pub fn as_pair(&self) -> (String, String) {
    (self.url.to_owned(), self.name.to_owned())
  }
}

impl fmt::Display for Dependency {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.url)
  }
}

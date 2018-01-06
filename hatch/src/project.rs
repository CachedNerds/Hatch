use std::fmt;

#[derive(Debug)]
pub struct Project {
  name: String,
  kind: ProjectKind,
  version: String,
}

impl Project {
  pub fn new(name: String, kind: ProjectKind, version: String) -> Project {
    Project { name, kind, version }
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

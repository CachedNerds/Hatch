use std::fmt;

use dtl::project as project_imp;

pub struct Project (project_imp::Project);

impl Project { 
  pub fn new(name: String, kind: ProjectKind, path: String, version: String) -> Project {
    Project(project_imp::Project::new(name, kind, path, version))
  }

  pub fn name(&self) -> &str {
    self.0.name()
  }

  pub fn kind(&self) -> &ProjectKind {
    self.0.kind()
  }
  
  pub fn path(&self) -> &str {
    self.0.path()
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

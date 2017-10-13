use std::fmt;
use std::path::{ PathBuf };
use tup::{ Manifest };

#[derive(Debug)]
pub enum LibraryKind {
  SharedLib,
  StaticLib,
}

#[derive(Debug)]
pub enum ProjectKind {
  Binary,
  Library(LibraryKind), 
}

impl ProjectKind {
  pub fn from_str(string: &str) -> ProjectKind {
    match string {
      "shared" => ProjectKind::Library(LibraryKind::SharedLib),
      "static" => ProjectKind::Library(LibraryKind::StaticLib),
      _ => ProjectKind::Binary,
    }
  }
}

impl fmt::Display for ProjectKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ProjectKind::Library(LibraryKind::SharedLib) => write!(f, "shared"),
      &ProjectKind::Library(LibraryKind::StaticLib) => write!(f, "static"),
      &ProjectKind::Binary => write!(f, "binary"),
    }
  }
}

#[derive(Debug)]
pub struct Project {
  name: String,
  build_type: ProjectKind,
  path: String,
}

impl Project {
  pub fn new(name: String, build_type: ProjectKind, path: String) -> Project {
    Project { name, build_type, path }
  }

  pub fn name(&self) -> &str {
    &self.name.as_str()
  }

  pub fn path(&self) -> &str {
    &self.path.as_str()
  }

  pub fn build_type(&self) -> &ProjectKind {
    &self.build_type
  }
}

use std::fmt;
use std::path::{ PathBuf };

#[derive(Debug)]
pub enum LibraryKind {
  Shared,
  Static,
}

impl fmt::Display for LibraryKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      &Shared => write!(f, "shared"),
      &Static => write!(f, "static"),
    }
  }
}

#[derive(Debug)]
pub enum ProjectKind {
  Binary,
  Library(LibraryKind),
}

impl ProjectKind {
  pub fn from_str(string: &str) -> ProjectKind {
    match string {
      "shared" => ProjectKind::Library(LibraryKind::Shared),
      "static" => ProjectKind::Library(LibraryKind::Static),
      _ => ProjectKind::Binary,
    }
  }
}

impl fmt::Display for ProjectKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ProjectKind::Binary => write!(f, "Binary"),
      &ProjectKind::Library(LibraryKind::Shared) => write!(f, "Shared"),
      &ProjectKind::Library(LibraryKind::Static) => write!(f, "Static"),
    }
  }
}

#[derive(Debug)]
pub struct Project {
  name: String,
  build_type: ProjectKind,
  path: PathBuf,
}

impl Project {
  pub fn new(name: String, build_type: ProjectKind, path: PathBuf) -> Project {
    Project { name, build_type, path }
  }

  pub fn get_path(&mut self) -> &mut PathBuf {
    &mut self.path
  }
}

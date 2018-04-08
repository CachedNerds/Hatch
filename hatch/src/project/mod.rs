//pub mod build;
//
//use std::fmt;
//use std::path::PathBuf;
//use self::build::Config;
//use deps::dependency::Dependency;
//
//#[derive(Debug)]
//pub struct Project {
//  name: String,
//  version: String,
//  config: Config,
//  deps: Vec<Dependency>,
//  path: PathBuf
//}
//
//impl Project {
//  pub fn new(name: String,
//             version: String,
//             config: Config,
//             deps: Vec<Dependency>,
//             path: PathBuf) -> Project
//  {
//    Project {
//      name,
//      version,
//      config,
//      deps,
//      path
//    }
//  }
//
//  pub fn name(&self) -> &str {
//    self.name.as_ref()
//  }
//
//  pub fn version(&self) -> &str {
//    self.version.as_ref()
//  }
//
//  pub fn config(&self) -> &Config {
//    self.config.as_ref()
//  }
//
//  pub fn deps(&self) -> &Vec<Dependency> {
//    self.deps.as_ref()
//  }
//
//  pub fn path(&self) -> &PathBuf {
//    &self.path
//  }
//}
// ////#[derive(Debug)]
////pub enum LibraryKind { Shared, Static }
//

use cli::commands::{ BIN, STATIC, SHARED, HEADER };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectKind { Binary, Static, Shared, HeaderOnly }

impl  ProjectKind {
  pub fn from_str(project_kind_str :&str) -> ProjectKind {
    match project_kind_str {
      arg if arg == BIN => ProjectKind::Binary,
      arg if arg == STATIC => ProjectKind::Static,
      arg if arg == SHARED => ProjectKind::Shared,
      arg if arg == HEADER => ProjectKind::HeaderOnly,
      _ => ProjectKind::Static
    }
  }

  pub fn default() -> ProjectKind {
    ProjectKind::Static
  }
}

//impl AsRef<ProjectKind> for ProjectKind {
//  fn as_ref(&self) -> &ProjectKind {
//    &self
//  }
//}
//
////impl fmt::Display for ProjectKind {
////  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
////    match *self {
////      ProjectKind::Library(LibraryKind::Shared) => write!(f, "shared-lib"),
////      ProjectKind::Library(LibraryKind::Static) => write!(f, "static-lib"),
////      ProjectKind::Binary => write!(f, "binary"),
////    }
////  }
////}

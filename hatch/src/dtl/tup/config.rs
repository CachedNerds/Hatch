use project::{ LibraryKind, ProjectKind };

pub struct Config {
  project: String,
  lib_type: String,
}

impl Config {
  pub fn new(name: &str, project_kind: &ProjectKind) -> Config {
    Config { project: Config::project(name), lib_type: Config::lib_type(project_kind)}
  }

  pub fn project(name: &str) -> String {
    String::from("PROJECT = ".to_owned() + name)
  }

  pub fn lib_type(project_kind: &ProjectKind) -> String {
    let kind = match *project_kind {
      ProjectKind::Binary => "BINARY",
      ProjectKind::Library(LibraryKind::Static) => "STATIC",
      ProjectKind::Library(LibraryKind::Shared) => "SHARED"
    };

    String::from("LIB_TYPE = ".to_owned() + kind)
  }
}

impl ToString for Config {
  fn to_string(&self) -> String {
    [self.project.as_str(), self.lib_type.as_str()].join("\n")
  }
}
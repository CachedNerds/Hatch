use project::{ LibraryKind, ProjectKind };

pub struct Config {
  project: String,
  lib_type: String,
}

impl Config {
  pub fn new(name: &str, project_kind: &ProjectKind) -> Config {
    Config { project: Config::project(name), lib_type: Config::lib_type(project_kind) }
  }

  pub fn name() -> String {
    String::from("config.tup")
  }

  pub fn project(name: &str) -> String {
    String::from("PROJECT = ") + name
  }

  pub fn lib_type(project_kind: &ProjectKind) -> String {
    let kind = match *project_kind {
      ProjectKind::Binary => "binary",
      ProjectKind::Library(LibraryKind::Static) => "static",
      ProjectKind::Library(LibraryKind::Shared) => "shared"
    };

    String::from("LIB_TYPE = ") + kind
  }
}

impl ToString for Config {
  fn to_string(&self) -> String {
    [self.project.as_str(), self.lib_type.as_str()].join("\n")
  }
}
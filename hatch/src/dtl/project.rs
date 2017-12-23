use project::ProjectKind;
use cli::Cli;

pub struct Project {
  name: String,
  kind: ProjectKind,
  path: String,
  version: String,
}

impl Project {
  pub fn new(name: String, kind: ProjectKind, path: String, version: String) -> Project {
    Project { name, kind, path, version }
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  pub fn kind(&self) -> &ProjectKind {
    self.kind.as_ref()
  }

  pub fn path(&self) -> &str {
    self.path.as_ref()
  }
  
  pub fn version(&self) -> &str {
    self.version.as_ref()
  }
}

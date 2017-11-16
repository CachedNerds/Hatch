use project::ProjectKind;
use cli::Cli;

pub struct Project {
  name: String,
  kind: ProjectKind,
  path: String,
}

impl Project {
  pub fn new() -> Project {
    let cli = Cli::new();
    Project { name: cli.name(), kind: cli.kind(), path: cli.path() }
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
}

use project::ProjectKind;

pub struct Project {
  name: String,
  kind: ProjectKind,
  path: String,
}

impl Project {
  pub fn new(name: String, kind: ProjectKind, path: String) -> Project {
    Project { name, kind, path }
  }
}

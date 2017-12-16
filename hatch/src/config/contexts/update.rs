use std::fs::OpenOptions;

use project::Project;
use manifest::Manifest;
use config::Context;

pub struct Update {
  project: Project,
}

impl Context for Update {
  fn build_manifest(self) -> Manifest {
    Manifest::new(self.project)
  }
}

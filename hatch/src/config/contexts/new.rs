use std::fs::OpenOptions;

use project::Project;
use manifest::Manifest;
use config::Context;

pub struct New {
  project: Project,
}

impl Context for New {
  fn build_manifest(self) -> Manifest {
    Manifest::new(self.project)
  }
}

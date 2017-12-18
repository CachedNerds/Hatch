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

impl Update {
  pub fn new(project: Project) -> Update {
    Update {
      project
    }
  }
}

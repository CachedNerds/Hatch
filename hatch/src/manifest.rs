use asset::Asset;
use project::Project;

pub struct Manifest {
  project: Project,
  assets: Asset,
}

impl Manifest {
  pub fn new() -> Manifest {
    Manifest {
      project: Project::new(),
      assets: Asset::new(),
    }
  }
}

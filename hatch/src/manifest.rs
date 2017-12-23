use asset::Asset;
use project::Project;
use project::ProjectKind;
use project::LibraryKind;

pub struct Manifest {
  project: Project,
  assets: Asset
}

impl Manifest {
  pub fn new(project: Project) -> Manifest {
    Manifest {
      project: project,
      assets: Asset::new()
    }
  }
}

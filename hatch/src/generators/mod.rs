use project::Project;
use hatch_error::HatchResult;
use std::path::PathBuf;

pub mod tup;

pub trait Generator {
  // TODO: this interface should take references
  fn generate_assets(&self, project_path: PathBuf, project: &Project) -> HatchResult<()>;
}

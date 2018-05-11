use project::Project;
use hatch_error::HatchResult;
use std::path::PathBuf;
use generators::tup::Tup;

pub mod tup;

pub trait Generator {
  // TODO: this interface should take references
  fn generate_assets(&self, project_path: PathBuf, project: &Project) -> HatchResult<()>;
}

pub fn get_generator(generator: Option<String>) -> Option<Box<Generator>> {
  match generator {
    _ => Some(Box::new(Tup{}) as Box<Generator>),
  }
}
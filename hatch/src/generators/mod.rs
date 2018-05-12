use project::Project;
use hatch_error::HatchResult;
use std::path::PathBuf;
use generators::tup::Tup;
use constants::TUP;

pub mod tup;

pub trait Generator {
  // TODO: this interface should take references
  fn generate_assets(&self, project_path: PathBuf, project: &Project) -> HatchResult<()>;
}

pub fn get_generator(generator: &Option<String>) -> Option<Box<Generator>> {
  match generator.as_ref().map(|s| &s[..]) {
    Some(s) if s == TUP => Some(Box::new(Tup{}) as Box<Generator>),
    _ => Some(Box::new(Tup{}) as Box<Generator>),
  }
}

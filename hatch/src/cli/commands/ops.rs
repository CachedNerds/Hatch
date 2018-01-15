use hatch_error::HatchResult;
use project::Project;

pub trait ProjectOps {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<HatchResult<Project>>;
}

use project::Project;
use hatch_error::HatchError;

pub trait ProjectOps {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<Result<Project, HatchError>>;
}

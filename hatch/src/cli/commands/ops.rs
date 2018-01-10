use HatchResult;
use project::Project;
use clap::ArgMatches;

pub trait ProjectOps {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<HatchResult<Project>>;
}

pub trait RepoManager<'ops> {
  fn fetch_dependencies(&self, repo_spec: &Vec<(String, String)>, path: &str);
  fn get_includes(&self, args: &ArgMatches<'ops>) -> Vec<(String, String)>;
  fn modules_path(&self, base: &str) -> String {
    base.to_owned() + "/hatch_modules"
  }
}

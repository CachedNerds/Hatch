use clap::ArgMatches;
use git2::Repository;

pub trait RepoManager<'ops> {
  fn fetch_dependencies(&self, repo_spec: &Vec<(String, String)>, path: &str);
  fn get_includes(&self, args: &ArgMatches<'ops>) -> Vec<(String, String)>;
  fn modules_path(&self, base: &str) -> String {
    base.to_owned() + "/hatch_modules"
  }
}

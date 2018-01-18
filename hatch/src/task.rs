use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::generator;
use yaml;
use hatch_error::HatchResult;
use std::path::Path;

pub fn read_project(path: String) -> HatchResult<Project> {
  yaml::parse(Path::new(&path))
}

pub fn generate_assets(project: &Project) -> HatchResult<()> {
  generator::generate_all(AssetBuilder::from(project).assets())
}
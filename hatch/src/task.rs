use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::generator;
use assets::Asset;

pub fn generate_assets(project: &Project) {
  generator::generate_all(AssetBuilder::from(project).assets());
}
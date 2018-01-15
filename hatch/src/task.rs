use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::generator;
use assets::Asset;

pub fn generate_assets(project: &Project) {
  let mut asset_builder = AssetBuilder::from(project);
  for asset in asset_builder.assets() {
    println!("{}\n", asset.contents());
  }

  generator::generate_all(asset_builder.assets());
}
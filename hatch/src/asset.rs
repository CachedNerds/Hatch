use dtl::asset as asset_imp;
use dtl::asset::{ AssetKind };

#[derive(Debug)]
pub struct Asset(asset_imp::AssetBuilder);

impl Asset {
  pub fn new() -> Asset {
    Asset(asset_imp::AssetBuilder::new())
  }

  pub fn assets(&mut self) -> &Vec<AssetKind> {
    self.0.assets()
  }
}

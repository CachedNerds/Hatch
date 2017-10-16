use dtl::asset as asset_imp;

pub struct AssetOptions(asset_imp::AssetOptions);

impl AssetOptions {
  pub fn new() -> AssetOptions {
    AssetOptions(asset_imp::AssetOptions::new())
  }
}

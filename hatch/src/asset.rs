use dtl::asset as asset_imp;
use dtl::asset::{ AssetKind };

#[derive(Debug)]
pub struct Asset(asset_imp::AssetBuilder);

impl Asset {
  pub fn new() -> Asset {
    Asset(asset_imp::AssetBuilder::new())
  }

  pub fn build_tuprules(&mut self) -> &mut Asset {
    self.0.build_tuprules(); self
  }

  pub fn build_tupfile(&mut self) -> &mut Asset {
    self.0.build_tupfile(); self
  }

  pub fn build_project_config(&mut self) -> &mut Asset {
    self.0.build_project_config(); self
  }

  pub fn build_project_tupfile(&mut self) -> &mut Asset {
    self.0.build_project_tupfile(); self
  }

  pub fn build_project_test_tupfile(&mut self) -> &mut Asset {
    self.0.build_project_test_tupfile(); self
  }

  pub fn build_linux_platform(&mut self) -> &mut Asset {
    self.0.build_linux_platform(); self
  }

  pub fn build_darwin_platform(&mut self) -> &mut Asset {
    self.0.build_darwin_platform(); self
  }

  pub fn build_win32_platform(&mut self) -> &mut Asset {
    self.0.build_win32_platform(); self
  }
}

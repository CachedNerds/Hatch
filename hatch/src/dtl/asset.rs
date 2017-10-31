use dtl::tup::platform::{ PlatformAssets };
use dtl::tup::build_system::{ BuildAssets };
use dtl::tup::project::{ ProjectAssets };
use dtl::tup::test::{ TestAssets };

use dtl::tup::{ Assets };

use command::{ Command };

impl Command<Assets> for TupKind {
  fn execute(&self) -> Assets {
    match self {
      Tuprules => BuildAssets::tuprules(),
      ProjectConfig => ProjectAssets::config(),
      ProjectTupfile => ProjectAssets::tupfile(),
      ProjectTestTupfile => TestAssets::tupfile(),
    }
  }
}

#[derive(Debug)]
pub enum TupKind {
  Tuprules,
  ProjectConfig,
  ProjectTupfile,
  ProjectTestTupfile,
}

impl Command<Assets> for PlatformKind {
  fn execute(&self) -> Assets {
    match self {
      Linux => PlatformAssets::linux(),
      Darwin => PlatformAssets::darwin(),
      Win32 => PlatformKind::win32(),
    }
  }
}

#[derive(Debug)]
pub enum PlatformKind {
  Linux,
  Darwin,
  Win32,
}

impl Command<Assets> for AssetKind {
  fn execute(&self) -> Assets {
    match self {
      AssetKind::Os(ref platform) => platform.execute(),
      AssetKind::Tup(ref tup) => tup.execute(),
    }
  }
}

#[derive(Debug)]
pub enum AssetKind {
  Os(PlatformKind),
  Tup(TupKind),
}

#[derive(Debug)]
pub struct AssetBuilder {
  assets: Vec<AssetKind>,
}

impl AssetBuilder {
  pub fn new() -> AssetBuilder {
    AssetBuilder {
      assets: Vec::new(),
    }
  }

  pub fn paths() -> &'static str {
    "/C++/libs\n/C++/Toolbox\n/test"
  }

  pub fn build_tuprules(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::Tuprules));
  }

  pub fn build_tupfile(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::Tupfile));
  }

  pub fn build_project_config(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::ProjectConfig));
  }

  pub fn build_project_tupfile(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::ProjectTupfile));
  }

  pub fn build_project_test_tupfile(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::ProjectTestTupfile));
  }

  pub fn build_linux_platform(&mut self) {
    self.assets.push(AssetKind::Os(PlatformKind::Linux));
  }

  pub fn build_darwin_platform(&mut self) {
    self.assets.push(AssetKind::Os(PlatformKind::Darwin));
  }

  pub fn build_win32_platform(&mut self) {
    self.assets.push(AssetKind::Os(PlatformKind::Win32));
  }
}

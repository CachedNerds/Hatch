use command::{ Command };

#[derive(Debug)]
pub enum TupKind {
  Tuprules,
  Tupfile,
  ProjectConfig,
  ProjectTupfile,
  ProjectTestTupfile,
}

#[derive(Debug)]
pub enum PlatformKind {
  Linux,
  Darwin,
  Win32,
}

impl Command for AssetKind {
  fn execute(&self) {
    println!("{:?}", self);
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

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
  name: String,
  path: String,
  assets: Vec<AssetKind>,
}

impl AssetBuilder {
  pub fn new() -> AssetBuilder {
    AssetBuilder {
      name: String::new(),
      path: String::new(),
      assets: Vec::new(),
    }
  }

  pub fn build_tuprules(&mut self) {
    self.assets.push(AssetKind::Tup(TupKind::Tuprules));
  }
}

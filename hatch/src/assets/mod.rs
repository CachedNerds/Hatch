pub mod builder;
pub mod generator;
pub mod config;
pub mod tupfile;
pub mod platform;
pub mod tuprules;
pub mod test_tupfile;
mod tests;

use std::fmt;

pub trait Asset {
  fn path(&self) -> &str;
  fn name(&self) -> &str;
  fn contents(&self) -> &str;
}

pub fn print_file_path<T>(asset: T) where T: Asset {
  println!("{}", asset.path());
}

pub fn print_file_name<T>(asset: T) where T: Asset {
  println!("{}", asset.name());
}

pub fn print_file_contents<T>(asset: T) where T: Asset {
  println!("{}", asset.contents());
}

#[derive(Debug)]
pub enum TupKind { Tuprules, Config, Tupfile, TestTupfile }

#[derive(Debug)]
pub enum PlatformKind { Linux, Darwin, Windows }

#[derive(Debug)]
pub enum Arch { X64, X32 }

pub struct ProjectAsset {
  path: String,
  name: String,
  contents: String,
}

impl ProjectAsset {
  pub fn new(path: String, name: String, contents: String) -> ProjectAsset {
    ProjectAsset { path, name, contents }
  }
}

impl Asset for ProjectAsset {
  fn path(&self) -> &str {
    &self.path.as_str()
  }

  fn name(&self) -> &str {
    &self.name.as_str()
  }

  fn contents(&self) -> &str {
    &self.contents.as_str()
  }
}

impl fmt::Debug for ProjectAsset {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "path: {}, name: {}, contents: {}", self.path, self.name, self.contents)
  }
}

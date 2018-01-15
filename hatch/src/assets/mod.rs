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
  fn file_path(&self) -> &str;
  fn contents(&self) -> &str;
}

pub fn print_file_path<T>(asset: T) where T: Asset {
  println!("{}", asset.file_path());
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
  file_path: String,
  file_contents: String,
}

impl ProjectAsset {
  pub fn new(file_path: String, file_contents: String) -> ProjectAsset {
    ProjectAsset { file_path, file_contents }
  }
}

impl Asset for ProjectAsset {
  fn file_path(&self) -> &str {
    &self.file_path.as_str()
  }

  fn contents(&self) -> &str {
    &self.file_contents.as_str()
  }
}

impl fmt::Debug for ProjectAsset {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "path: {}, contents: {}", self.file_path, self.file_contents)
  }
}

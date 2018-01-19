pub mod builder;
pub mod generator;
pub mod config;
pub mod tupfile;
pub mod platform;
pub mod tuprules;
pub mod test_tupfile;
pub mod tupfile_ini;
mod tests;

use std::fmt;
use std::cmp;
use std::path::PathBuf;

pub trait Asset {
  fn path(&self) -> &PathBuf;
  fn name(&self) -> &str;
  fn contents(&self) -> &str;
}

pub fn print_file_path<T>(asset: T) where T: Asset {
  println!("{:?}", asset.path());
}

pub fn print_file_name<T>(asset: T) where T: Asset {
  println!("{}", asset.name());
}

pub fn print_file_contents<T>(asset: T) where T: Asset {
  println!("{}", asset.contents());
}

#[derive(Debug)]
pub enum TupKind { Tuprules, Config, Tupfile, TestTupfile, TupfileIni }

#[derive(Debug)]
pub enum PlatformKind { Linux, MacOS, Windows }

#[derive(Debug)]
pub enum Arch { X64, X32 }

pub struct ProjectAsset {
  path: PathBuf,
  name: String,
  contents: String,
}

impl ProjectAsset {
  pub fn new(path: PathBuf, name: String, contents: String) -> ProjectAsset {
    ProjectAsset { path, name, contents }
  }
}

impl Asset for ProjectAsset {
  fn path(&self) -> &PathBuf {
    &self.path
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
    write!(f, "path: {:?}, name: {}, contents: {}", self.path, self.name, self.contents)
  }
}

impl cmp::PartialEq for ProjectAsset {
  fn eq(&self, other: &ProjectAsset) -> bool {
    self.name == other.name && self.contents == other.contents
  }
}

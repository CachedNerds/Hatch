pub mod builder;
pub mod generator;
pub mod janitor;
pub mod config;
pub mod tupfile;
pub mod platform;
pub mod tuprules;
pub mod test_tupfile;
pub mod tupfile_ini;
pub mod catch_header;
pub mod catch_definition;

#[cfg(test)]
mod tests;

use std::fmt;
use std::cmp;
use std::path::{ Path, PathBuf };

pub trait Asset {
  fn path(&self) -> &Path;
  fn name(&self) -> &str;
  fn contents(&self) -> &str;
}

pub fn print_file_path<T>(asset: T) where T: Asset {
  println!("{}", asset.path().display());
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
  fn path(&self) -> &Path {
    self.path.as_path()
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
    write!(f, "path: {}, name: {}, contents: {}", self.path.display(), self.name, self.contents)
  }
}

impl cmp::PartialEq for ProjectAsset {
  fn eq(&self, other: &ProjectAsset) -> bool {
    self.name == other.name && self.contents == other.contents
  }
}

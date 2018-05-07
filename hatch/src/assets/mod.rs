use std::path::Path;
use std::path::PathBuf;
use std::fs;
use failure::ResultExt;
use hatch_error::HatchResult;
use std::io::Write;
use core::cmp;
use std::fmt;

pub mod builder;
pub mod generator;
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
//
//use std::fmt;
//use std::cmp;
//use std::path::{ Path };
//use project::Project;
//
//pub trait Asset {
//  fn path(&self) -> &Path;
//  fn name(&self) -> &str;
//  fn contents(&self) -> &str;
//}
//
//pub fn print_file_path<T>(asset: T) where T: Asset {
//  println!("{}", asset.path().display());
//}
//
//pub fn print_file_name<T>(asset: T) where T: Asset {
//  println!("{}", asset.name());
//}
//
//pub fn print_file_contents<T>(asset: T) where T: Asset {
//  println!("{}", asset.contents());
//}

#[derive(Debug)]
pub enum TupKind {
    Tuprules,
    Config,
    Tupfile,
    TestTupfile,
    TupfileIni,
}

#[derive(Debug)]
pub enum PlatformKind {
    Linux,
    MacOS,
    Windows,
}

pub struct ProjectAsset {
    path: PathBuf,
    name: String,
    contents: String,
}

impl ProjectAsset {
  pub fn new(path: PathBuf, name: String, contents: String) -> ProjectAsset {
    ProjectAsset { path, name, contents }
  }

  pub fn path(& self) -> &Path {
    self.path.as_path()
  }

  pub fn write(&self) -> HatchResult<()> {
    let path = self.path();
    fs::create_dir_all(path).with_context(|e| {
    format!("Failed to create directory: `{}` : {}", path.display(), e)
    })?;

    let file_path = path.join(&self.name);
    let mut file = fs::File::create(&file_path).with_context(|e| {
    format!("Failed to create file: `{}` : {}", file_path.display(), e)
    })?;

    file.write_all(self.contents.as_bytes()).with_context(|e| {
    format!("Failed to write contents to file: `{}` : {}", file_path.display(), e)
    })?;

    Ok(())
  }
}

//impl<'project_asset> ProjectAsset<'project_asset> {
//  pub fn new(path: &'project_asset Path, name: &'project_asset str, contents: &'project_asset str) -> ProjectAsset<'project_asset> {
//    ProjectAsset { path, name, contents }
//  }
//}

//impl<'asset> Asset for ProjectAsset<'asset> {
//  fn path(& self) -> &Path {
//    self.path
//  }
//
//  fn name(&self) -> &str {
//    &self.name
//  }
//
//  fn contents(&self) -> &str {
//    &self.contents
//  }
//}
//
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

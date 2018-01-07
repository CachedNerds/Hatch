use project::Project;
use dtl::tup::{ Assets };

use dtl::tup::build_system::BuildAssets;
use dtl::tup::project::ProjectAssets;
use dtl::tup::platform::PlatformAssets;
use dtl::tup::test::TestAssets;

pub fn print_file_path<T>(asset: T) where T: Assets {
  println!("{}", asset.path());
}

pub fn print_file_contents<T>(asset: T) where T: Assets {
  println!("{}", asset.path());
}

#[derive(Debug)]
pub enum TupKind { Tuprules, ProjectConfig, ProjectTupfile, ProjectTestTupfile }

#[derive(Debug)]
pub enum PlatformKind { Linux, Darwin, Win32 }

#[derive(Debug)]
pub enum AssetKind { Os(PlatformKind), Tup(TupKind) }

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

  pub fn assets(&mut self) -> &Vec<AssetKind> {
    &self.assets.as_ref()
  }

  fn generate_tupkind_assets(&self, asset: &TupKind, project: &Project) {
    match * asset {
      TupKind::Tuprules => print_file_path(BuildAssets::tuprules()),
      TupKind::ProjectConfig => print_file_path(ProjectAssets::config(&project)),
      TupKind::ProjectTupfile => print_file_path(ProjectAssets::tupfile(&project)),
      TupKind::ProjectTestTupfile => print_file_path(TestAssets::tupfile(&project)),
    }
  }

  fn generate_platformkind_assets(&self, asset: &PlatformKind) {
    match * asset {
      PlatformKind::Linux => print_file_path(PlatformAssets::linux()),
      PlatformKind::Darwin => print_file_path(PlatformAssets::darwin()),
      PlatformKind::Win32 => print_file_path(PlatformAssets::win32()),
    }
  }
}

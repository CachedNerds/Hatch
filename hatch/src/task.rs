use project::Project;
use assets::builder::Builder as AssetBuilder;
use std::path::Path;
use hatch_error::HatchResult;
use assets::generator;
use assets::PlatformKind;
use repo::hatchfile_path;
use yaml;
use os_info;
use os_info::Type::{ Macos, Windows };

pub fn read_project(path: &Path) -> HatchResult<Project> {
  yaml::parse(hatchfile_path(path).as_path())
}

pub fn generate_assets(project: &Project) -> HatchResult<()> {
  generator::generate_all(AssetBuilder::from(project).assets())
}

pub fn platform_type() -> PlatformKind {
  match *os_info::get().os_type() {
    Macos => PlatformKind::MacOS,
    Windows => PlatformKind::Windows,
    _ => PlatformKind::Linux
  }
}

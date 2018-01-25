use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::generator;
use assets::PlatformKind;
use yaml;
use hatch_error::HatchResult;
use std::path::{ PathBuf, Path };
use os_info;
use os_info::Type;
use repo::hatchfile_path;

pub fn read_project(path: &Path) -> HatchResult<Project> {
  yaml::parse(hatchfile_path(path).as_path())
}

pub fn generate_assets(project: &Project) -> HatchResult<()> {
  generator::generate_all(AssetBuilder::from(project).assets())
}

pub fn platform_type() -> PlatformKind {
  let info = os_info::get();
    match *info.os_type() {
      Type::Macos => PlatformKind::MacOS,
      Type::Windows => PlatformKind::Windows,
      _ => PlatformKind::Linux
    }
}

use project::{ Project, Arch };
use assets::builder::Builder as AssetBuilder;
use std::path::Path;
use hatch_error::{ HatchResult, ResultExt };
use assets::generator;
use assets::PlatformKind;
use yaml;
use os_info;
use os_info::Type::{ Macos, Windows };
use std::option::Option;
use std::mem::size_of;


pub fn read_project(path: &Path) -> HatchResult<Project> {
  let project = yaml::parse(path, String::from("Hatch.yml")).with_context(|e| {
    format!("failed to read project at `{}` : {}", path.display(), e)
  })?;

  Ok(project)
}

pub fn generate_assets(project: &Project) -> HatchResult<()> {
  generator::generate_all(AssetBuilder::from(project).assets()).with_context(|e| {
    format!("asset generation failed : `{}`", e)
  })?;

  Ok(())
}

pub fn platform_type() -> PlatformKind {
  match *os_info::get().os_type() {
    Macos => PlatformKind::MacOS,
    Windows => PlatformKind::Windows,
    _ => PlatformKind::Linux
  }
}

pub fn architecture() -> Option<Arch> {
  match size_of::<&char>() {
    32 => Some(Arch::X32),
    64 => Some(Arch::X64),
    _ => None
  }
}

#[cfg(test)]
mod tests {

}

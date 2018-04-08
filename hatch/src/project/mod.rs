use platform::arch::Arch;
use deps::dependency::Dependency;
use project::ProjectKind;

use cli::commands::{ BIN, STATIC, SHARED, HEADER };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectKind { Binary, Static, Shared, HeaderOnly }

impl  ProjectKind {
  pub fn from_str(project_kind_str :&str) -> ProjectKind {
    match project_kind_str {
      arg if arg == BIN => ProjectKind::Binary,
      arg if arg == STATIC => ProjectKind::Static,
      arg if arg == SHARED => ProjectKind::Shared,
      arg if arg == HEADER => ProjectKind::HeaderOnly,
      _ => ProjectKind::Static
    }
  }
  pub fn default() -> ProjectKind {
    ProjectKind::Static
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Target { Debug, Release }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompilerOptions {
  pub kind: ProjectKind,
  pub compiler: String,
  pub compiler_flags: String,
  pub linker_flags: String,
  pub arch: Arch,
  pub target: Target,
}

impl CompilerOptions {
  pub fn new(kind: ProjectKind, compiler: String, compiler_flags: String, linker_flags: String,
             arch: Arch, target: Target) -> CompilerOptions {
    CompilerOptions {
      kind,
      compiler,
      compiler_flags,
      linker_flags,
      arch,
      target
    }
  }

  pub fn default_for_kind(kind: ProjectKind) -> CompilerOptions {
    let compiler: String = String::from("g++");
    let compiler_flags= String::from("-c");
    let linker_flags= String::from("-v");
    let mut arch: Arch = Arch::X64;
    if let Some(architecture) = Arch::architecture() {
      arch = architecture;
    }
    let target: Target = Target::Debug;

    CompilerOptions {
      kind,
      compiler,
      compiler_flags,
      linker_flags,
      arch,
      target
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
  pub name : String,
  pub version: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub config: Option<CompilerOptions>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub includes: Vec<Dependency>,
}

impl Project {
  pub fn new(name: String, version: String, config: Option<CompilerOptions>,
             includes: Vec<Dependency>) -> Project {
    Project { name, version, config, includes }
  }
}
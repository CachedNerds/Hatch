pub mod compiler_options;
pub mod project_kind;
pub mod target;

pub use self::compiler_options::CompilerOptions;
pub use self::project_kind::ProjectKind;
pub use self::target::Target;

use deps::dependency::Dependency;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
  pub name : String,
  pub version: String,
  pub kind: ProjectKind,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub config: Option<CompilerOptions>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub includes: Vec<Dependency>,
}

impl Project {
  pub fn new(name: String, version: String, kind: ProjectKind, config: Option<CompilerOptions>,
             includes: Vec<Dependency>) -> Project {
    Project { name, version, kind, config, includes }
  }
}

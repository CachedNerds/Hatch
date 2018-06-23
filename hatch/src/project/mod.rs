pub mod compiler_options;
pub mod project_kind;
pub mod target;

pub use self::compiler_options::CompilerOptions;
pub use self::project_kind::ProjectKind;
pub use self::target::Target;

use deps::dependency::Dependency;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    name: String,
    version: String,
    kind: ProjectKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    compiler_options: Option<CompilerOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<Dependency>>,
}

impl Project {
    pub fn new(
        name: String,
        version: String,
        kind: ProjectKind,
        compiler_options: Option<CompilerOptions>,
        dependencies: Option<Vec<Dependency>>,
    ) -> Project {
        Project {
            name,
            version,
            kind,
            compiler_options,
            dependencies,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn version(&self) -> &str {
        self.version.as_str()
    }
    pub fn kind(&self) -> &ProjectKind {
        &self.kind
    }
    pub fn compiler_options(&self) -> &Option<CompilerOptions> {
        &self.compiler_options
    }
    pub fn dependencies(&self) -> &Option<Vec<Dependency>> {
        &self.dependencies
    }
}

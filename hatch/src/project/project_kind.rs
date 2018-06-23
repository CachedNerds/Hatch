use constants::{BIN, HEADER, SHARED, STATIC};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectKind {
    Binary,
    Static,
    Shared,
    HeaderOnly,
}

impl ProjectKind {
    pub fn from_str(project_kind_str: &str) -> ProjectKind {
        match project_kind_str {
            arg if arg == BIN => ProjectKind::Binary,
            arg if arg == STATIC => ProjectKind::Static,
            arg if arg == SHARED => ProjectKind::Shared,
            arg if arg == HEADER => ProjectKind::HeaderOnly,
            _ => ProjectKind::Static,
        }
    }
}

impl Default for ProjectKind {
    fn default() -> Self {
        ProjectKind::Static
    }
}

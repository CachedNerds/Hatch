pub mod build;

use std::fmt;
use std::path::PathBuf;
use self::build::Config;
use deps::dependency::Dependency;

#[derive(Debug)]
pub struct Project {
    name: String,
    version: String,
    config: Config,
    deps: Vec<Dependency>,
    path: PathBuf,
}

impl Project {
    pub fn new(
        name: String,
        version: String,
        config: Config,
        deps: Vec<Dependency>,
        path: PathBuf,
    ) -> Project {
        Project {
            name,
            version,
            config,
            deps,
            path,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    pub fn config(&self) -> &Config {
        self.config.as_ref()
    }

    pub fn deps(&self) -> &Vec<Dependency> {
        self.deps.as_ref()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Debug)]
pub enum LibraryKind {
    Shared,
    Static,
}

#[derive(Debug)]
pub enum ProjectKind {
    Binary,
    Library(LibraryKind),
}

impl AsRef<ProjectKind> for ProjectKind {
    fn as_ref(&self) -> &ProjectKind {
        &self
    }
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProjectKind::Library(LibraryKind::Shared) => write!(f, "shared-lib"),
            ProjectKind::Library(LibraryKind::Static) => write!(f, "static-lib"),
            ProjectKind::Binary => write!(f, "binary"),
        }
    }
}

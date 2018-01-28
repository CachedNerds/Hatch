use std::fmt;
use std::path::PathBuf;
use std::ffi::OsString;


#[derive(Debug)]
pub struct Project {
  name: String,
  version: String,
  config: BuildConfig,
  deps: Vec<Dependency>,
  path: PathBuf
}

impl Project {
  pub fn new(name: String,
             version: String,
             config: BuildConfig,
             deps: Vec<Dependency>,
             path: PathBuf) -> Project
  {
    Project {
      name,
      version,
      config,
      deps,
      path
    }
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  pub fn version(&self) -> &str {
    self.version.as_ref()
  }

  pub fn config(&self) -> &BuildConfig {
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
pub enum LibraryKind { Shared, Static }

#[derive(Debug)]
pub enum ProjectKind { Binary, Library(LibraryKind) }

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

#[derive(Debug)]
pub enum Arch { X64, X32 }

impl AsRef<Arch> for Arch {
  fn as_ref(&self) -> &Arch {
    &self
  }
}

impl fmt::Display for Arch {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Arch::X64 => write!(f, "x64"),
      Arch::X32 => write!(f, "x32"),
    }
  }
}

#[derive(Debug)]
pub enum Target { Debug, Release }

impl AsRef<Target> for Target {
  fn as_ref(&self) -> &Target {
    &self
  }
}

impl fmt::Display for Target {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Target::Debug => write!(f, "debug"),
      Target::Release => write!(f, "release"),
    }
  }
}

#[derive(Debug)]
pub struct BuildConfig {
  kind: ProjectKind,
  compiler: String,
  compiler_flags: Vec<String>,
  linker_flags: Vec<String>,
  arch: Arch,
  target: Target
}

impl BuildConfig {
  pub fn new(kind: ProjectKind,
             compiler: String,
             compiler_flags: Vec<String>,
             linker_flags: Vec<String>,
             arch: Arch,
             target: Target) -> BuildConfig
  {
    BuildConfig {
      kind,
      compiler,
      compiler_flags,
      linker_flags,
      arch,
      target
    }
  }

  pub fn kind(&self) -> &ProjectKind {
    self.kind.as_ref()
  }

  pub fn compiler(&self) -> &str {
    self.compiler.as_ref()
  }

  pub fn compiler_flags(&self) -> &Vec<String> {
    self.compiler_flags.as_ref()
  }

  pub fn linker_flags(&self) -> &Vec<String> {
    self.linker_flags.as_ref()
  }

  pub fn arch(&self) -> &Arch {
    self.arch.as_ref()
  }

  pub fn target(&self) -> &Target {
    self.target.as_ref()
  }
}

impl AsRef<BuildConfig> for BuildConfig {
  fn as_ref(&self) -> &BuildConfig {
    &self
  }
}

//impl Clone for BuildConfig {
//  fn clone(&self) -> BuildConfig {
//    *self
//  }
//}

#[derive(Debug)]
pub struct Dependency {
  url: String,
  name: String,
}

impl Dependency {
  pub fn new(url: String) -> Dependency {
    Dependency {
      name: PathBuf::from(OsString::from(url.clone()))
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .into_owned()
        .as_str()
        .replace(".git", ""),
      url,
    }
  }

  pub fn as_pair(&self) -> (String, String) {
    (self.url.to_owned(), self.name.to_owned())
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  pub fn url(&self) -> &str {
    self.url.as_ref()
  }
}

impl fmt::Display for Dependency {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.url)
  }
}

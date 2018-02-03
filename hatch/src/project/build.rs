use std::fmt;
use project::ProjectKind;
use platform::arch::Arch;

#[derive(Debug)]
pub struct Config {
  kind: ProjectKind,
  compiler: String,
  compiler_flags: Vec<String>,
  linker_flags: Vec<String>,
  arch: Arch,
  target: Target
}

impl Config {
  pub fn new(kind: ProjectKind,
             compiler: String,
             compiler_flags: Vec<String>,
             linker_flags: Vec<String>,
             arch: Arch,
             target: Target) -> Config
  {
    Config {
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

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    &self
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


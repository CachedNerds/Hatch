use platform::arch::Arch;
use project::Target;
use project::ProjectKind;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompilerOptions {
  compiler: String,
  compiler_flags: String,
  linker_flags: String,
  arch: Arch,
  target: Target,
}

impl CompilerOptions {
  pub fn new(compiler: String, compiler_flags: String, linker_flags: String,
             arch: Arch, target: Target) -> CompilerOptions {
    CompilerOptions {
      compiler,
      compiler_flags,
      linker_flags,
      arch,
      target
    }
  }

  pub fn default() -> CompilerOptions {
    let compiler: String = String::from("g++");
    let compiler_flags = String::from("-c");
    let linker_flags = String::from("-v");
    let mut arch: Arch = Arch::X64;
    if let Some(architecture) = Arch::architecture() {
      arch = architecture;
    }
    let target: Target = Target::Debug;

    CompilerOptions {
      compiler,
      compiler_flags,
      linker_flags,
      arch,
      target
    }
  }

  pub fn default_from_kind(kind: &ProjectKind) -> Option<CompilerOptions> {
    match kind {
      ProjectKind::HeaderOnly => None,
      _ => Some(CompilerOptions::default())
    }
  }
}
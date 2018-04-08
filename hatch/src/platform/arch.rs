use std::mem::size_of;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Arch { X64, X86 }

impl AsRef<Arch> for Arch {
  fn as_ref(&self) -> &Arch {
    &self
  }
}

impl fmt::Display for Arch {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Arch::X64 => write!(f, "x64"),
      Arch::X86 => write!(f, "x86"),
    }
  }
}

impl Arch {
  pub fn architecture() -> Option<Arch> {
    match size_of::<&char>() {
      32 => Some(Arch::X86),
      64 => Some(Arch::X64),
      _ => None
    }
  }
}

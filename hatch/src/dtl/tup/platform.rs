use dtl::tup::{ Assets };

pub struct PlatformAssets {
  file_path: String,
  file_contents: String,
}

impl Assets for PlatformAssets {
  fn path(&self) -> &str {
    &self.file_path.as_str()
  }

  fn contents(&self) -> &str {
    &self.file_contents.as_str()
  }
}

impl PlatformAssets {
  // Expects path to be /xxxx/xxxx/Toolbox
  pub fn linux(path: &str) -> PlatformAssets {
    let file_path = path.to_string() + "/linux.tup";
    let file_contents =
"STATIC = a
SHARED = so".to_string();
    PlatformAssets { file_path, file_contents }
  }
  
  // Expects path to be /xxxx/xxxx/Toolbox
  pub fn darwin(path: &str) -> PlatformAssets {
    let file_path = path.to_string() + "/macosx.tup";
    let file_contents =
"STATIC = a
SHARED = so".to_string();
    PlatformAssets { file_path, file_contents }
  }

  // Expects path to be /xxxx/xxxx/Toolbox
  pub fn win32(path: &str) -> PlatformAssets {
    let file_path = path.to_string() + "/win32.tup";
    let file_contents =
"STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>".to_string();
    PlatformAssets { file_path, file_contents }
  }
}

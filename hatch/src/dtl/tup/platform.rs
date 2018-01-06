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
  pub fn linux() -> PlatformAssets {
    let file_path = "C++/libs/linux.tup".to_owned();
    let file_contents =
"STATIC = a
SHARED = so".to_string();
    PlatformAssets { file_path, file_contents }
  }
  
  pub fn darwin() -> PlatformAssets {
    let file_path = "C++/libs/macosx.tup".to_owned();
    let file_contents =
"STATIC = a
SHARED = so".to_string();
    PlatformAssets { file_path, file_contents }
  }

  pub fn win32() -> PlatformAssets {
    let file_path = "C++/libs/win32.tup".to_owned();
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

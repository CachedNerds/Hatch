pub struct PlatformAssets {}

impl PlatformAssets {
  pub fn linux() -> String {
"STATIC = a
SHARED = so".to_string()
  }
  
  pub fn darwin() -> String {
"STATIC = a
SHARED = so".to_string()
  }

  pub fn win32() -> String {
"STATIC = lib
SHARED = dll

# Use clang for front-end
CC = clang++.exe

# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>".to_string()
  }
}

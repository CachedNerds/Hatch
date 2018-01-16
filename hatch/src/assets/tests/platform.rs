use assets::platform::{ Linux, MacOS, Windows };

#[test]
fn build_linux() {
  assert_eq!("STATIC = a\nSHARED = so", Linux::new().to_string());
}

#[test]
fn build_macos() {
  assert_eq!("STATIC = a\nSHARED = so", MacOS::new().to_string());
}

#[test]
fn build_windows() {
  let windows_content = String::from(
"STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>");
  assert_eq!(windows_content, Windows::new().to_string());
}
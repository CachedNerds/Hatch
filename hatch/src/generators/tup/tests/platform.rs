use generators::tup::platform::linux::to_string as linux_to_string;
use generators::tup::platform::windows::to_string as windows_to_string;
use generators::tup::platform::mac_os::to_string as mac_os_to_string;


#[test]
fn build_linux() {
    assert_eq!("STATIC = a\nSHARED = so", linux_to_string());
}

#[test]
fn build_macos() {
    assert_eq!("STATIC = a\nSHARED = so", mac_os_to_string());
}

#[test]
fn build_windows() {
    let windows_content = String::from(
        "STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>",
    );
    assert_eq!(windows_content, windows_to_string());
}

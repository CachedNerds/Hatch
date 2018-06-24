use generators::tup::platform::linux;
use generators::tup::platform::mac_os;
use generators::tup::platform::windows;

#[test]
fn linux_file_name() {
    assert_eq!("linux.tup", linux::file_name())
}

#[test]
fn generate_linux() {
    assert_eq!("STATIC = a\nSHARED = so", linux::make_string());
}

#[test]
fn mac_os_file_name() {
    assert_eq!("macosx.tup", mac_os::file_name())
}

#[test]
fn generate_mac_os() {
    assert_eq!("STATIC = a\nSHARED = so", mac_os::make_string());
}

#[test]
fn windows_file_name() {
    assert_eq!("win32.tup", windows::file_name())
}

#[test]
fn generate_windows() {
    let windows_content = String::from(
        "STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>",
    );
    assert_eq!(windows_content, windows::make_string());
}

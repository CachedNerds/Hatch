use os_info;
use os_info::Type::{Macos, Windows};
use generators::platform_kind::PlatformKind;

pub fn platform_type() -> PlatformKind {
    match *os_info::get().os_type() {
        Macos => PlatformKind::MacOS,
        Windows => PlatformKind::Windows,
        _ => PlatformKind::Linux,
    }
}

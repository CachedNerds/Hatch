pub use self::platform::{ PlatformAssets };
pub use self::build_system::{ BuildAssets };
pub use self::project::{ ProjectAssets };
pub use self::test::{ TestAssets };

pub mod platform;
pub mod build_system;
pub mod project;
pub mod test;

pub trait Assets {
  fn contents(&self) -> &str;
  fn path(&self) -> &str;
}

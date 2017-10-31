pub mod build_system;
pub mod project;
pub mod platform;
pub mod test;

pub trait Assets {
  fn contents(&self) -> &str;
  fn path(&self) -> &str;
}

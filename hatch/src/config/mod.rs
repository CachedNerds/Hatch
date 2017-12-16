pub mod contexts;

use manifest::Manifest;  

pub trait Context {
  fn build_manifest() -> Manifest;
}

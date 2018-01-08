#![forbid(unsafe_code)]

pub type HatchResult<T> = Result<T, hatch_error::HatchError>;

#[macro_use]
pub extern crate clap;

pub extern crate yaml_rust;

pub extern crate git2;

pub mod project;
pub mod cli;
pub mod asset;
pub mod hatch_error;
pub mod yaml;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

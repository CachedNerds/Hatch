#![forbid(unsafe_code)]


#[macro_use]
pub extern crate clap;
pub extern crate yaml_rust;
pub extern crate git2;
#[macro_use]
pub extern crate failure;

pub mod project;
pub mod cli;
pub mod asset;
pub mod hatch_error;
pub mod yaml;
pub mod repo;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

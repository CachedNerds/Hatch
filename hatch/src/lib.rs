#![forbid(unsafe_code)]

#[macro_use]
pub extern crate clap;

pub extern crate yaml_rust;

pub mod project;
pub mod cli;
pub mod asset;
pub mod manifest;
pub mod config;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

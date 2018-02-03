#![forbid(unsafe_code)]


#[macro_use]
pub extern crate clap;
pub extern crate yaml_rust;
pub extern crate git2;
#[macro_use]
pub extern crate failure;
pub extern crate os_info;
pub extern crate reqwest;

pub mod project;
pub mod cli;
pub mod hatch_error;
pub mod yaml;
pub mod assets;
pub mod task;
pub mod deps;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

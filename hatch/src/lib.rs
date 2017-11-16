#![forbid(unsafe_code)]

#[macro_use]
pub extern crate clap;

pub mod project;
pub mod cli;
pub mod asset;
pub mod manifest;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

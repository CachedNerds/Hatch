#![forbid(unsafe_code)]

#[macro_use]
pub extern crate clap;

pub mod command;
pub mod project;
pub mod cli;
pub mod egg;
pub mod asset;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

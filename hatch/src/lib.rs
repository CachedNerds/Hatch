
#[macro_use]
extern crate clap;
pub use self::clap::*;

pub mod project;
pub mod cli;
mod dtl;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}

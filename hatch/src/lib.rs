#![forbid(unsafe_code)]

#[macro_use]
pub extern crate clap;
#[macro_use]
pub extern crate failure;
pub extern crate git2;
pub extern crate os_info;
pub extern crate reqwest;
pub extern crate yaml_rust;

pub mod project;
pub mod cli;
pub mod hatch_error;
pub mod yaml;
pub mod assets;
pub mod task;
pub mod deps;
pub mod platform;
pub mod locations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

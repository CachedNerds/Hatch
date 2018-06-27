#![forbid(unsafe_code)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
pub extern crate clap;
#[macro_use]
pub extern crate failure;
extern crate core;
pub extern crate git2;
pub extern crate os_info;
pub extern crate reqwest;
pub extern crate yaml_rust;
pub extern crate assert_fs;
pub extern crate predicates;

pub mod cli;
pub mod constants;
pub mod deps;
pub mod generators;
pub mod hatch_error;
pub mod locations;
pub mod platform;
pub mod project;

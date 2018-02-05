use config::{ ConfigLoader };
use std::collections::HashMap;
use std::path::Path;
use clap::ArgMatches;

pub struct CliConfigLoader<'loader> {
  args: ArgMatches<'loader>
}

impl<'loader> CliConfigLoader<'loader> {
  pub fn new(args: ArgMatches<'loader>) -> CliConfigLoader<'loader> {
    CliConfigLoader { args }
  }
}

impl<'config> ConfigLoader for CliConfigLoader<'config> {
  fn getConfig(&self, subcommand: &str) -> HashMap<String, String> {
    // parse the arguments from self.args for the specified subcommand into a
    // hashmap
    HashMap::new()
  }
}

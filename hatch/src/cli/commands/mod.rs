pub mod new;
pub mod update;

use clap::ArgMatches;

pub trait Command {
  fn execute<'a>(&self, args: &ArgMatches<'a>);
}
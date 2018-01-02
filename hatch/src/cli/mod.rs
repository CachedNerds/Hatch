pub mod commands;

use dtl::cli as cli_imp;
use clap::{ App, ArgMatches };

pub struct Cli<'cli>(cli_imp::Cli<'cli>);

impl<'cli>Cli<'cli> {
  pub fn new<I>(subcommands: I) -> Cli<'cli>
    where I: IntoIterator<Item=App<'cli, 'cli>> {
    Cli(cli_imp::Cli::new(subcommands))
  }

  pub fn subcommand(&self) -> (&str, Option<&ArgMatches<'cli>>) {
    self.0.subcommand()
  }
}

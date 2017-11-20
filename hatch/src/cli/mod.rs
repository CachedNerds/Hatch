pub mod commands;

use dtl::cli as cli_imp;
use project::{ ProjectKind };
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

  pub fn kind(&self) -> ProjectKind {
    self.0.build_type()
  }

  pub fn name(&self) -> String {
    self.0.name()
  }

  pub fn path(&self) -> String {
    self.0.path()
  }
}

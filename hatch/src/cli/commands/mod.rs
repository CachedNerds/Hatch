pub mod new;
pub mod update;

use clap::{ ArgMatches, App };

pub trait Command<'command> {
  fn cli_subcommand(&self) -> App<'command, 'command>;
  fn subcommand_name(&self) -> &'static str;
  fn execute(&self, args: &ArgMatches<'command>);
}

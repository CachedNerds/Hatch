use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

pub struct Update {
  name: &'static str
}

impl Update {
  pub fn new() -> Update {
    Update { name: "update" }
  }
}

impl<'update> Command<'update> for Update {
  fn cli_subcommand(&self) -> App<'update, 'update> {
    SubCommand::with_name(&self.name)
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Mackenzie Clark <mackenzie.a.z.c@gmail.com>")
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute<'a>(&self, args: &ArgMatches<'a>) {
    println!("{}", &self.name);
  }
}
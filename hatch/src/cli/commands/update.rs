use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

pub struct Update;

impl<'new> Update {
  pub fn subcmd() -> App<'new, 'new> {
    SubCommand::with_name(Update::subcmd_name())
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Mackenzie Clark <mackenzie.a.z.c@gmail.com>")
  }

  pub fn subcmd_name() -> &'static str {
    "update"
  }
}

impl Command for Update {
  fn execute<'a>(&self, args: &ArgMatches<'a>) {
    println!("{}", Update::subcmd_name())
  }
}
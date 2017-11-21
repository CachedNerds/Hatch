use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

pub struct New {
  name: &'static str
}

impl New {
  pub fn new() -> New {
    New { name: "new" }
  }
}

impl<'new> Command<'new> for New {
  fn cli_subcommand(&self) -> App<'new, 'new> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")

      .arg(Arg::with_name("TOOLBOX_PATH")
        .help("Path to toolbox. (default = ./)")
        .long("path").short("p").required(false).takes_value(true))

      .arg(Arg::with_name("PROJECT_NAME")
        .help("Name of project")
        .required(true).takes_value(true))

      .arg(Arg::with_name("bin")
        .help("Generate a stand alone executable")
        .long("bin").short("b").required(false)) 

      .arg(Arg::with_name("static")
        .help("Generate a static library")
        .long("static").short("s").conflicts_with("bin").required(false))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute<'a>(&self, args: &ArgMatches<'a>) {
    println!("{}", &self.name)
  }
}
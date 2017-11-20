use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

pub struct New;

impl<'new> New {
  pub fn subcmd() -> App<'new, 'new> {
    SubCommand::with_name(New::subcmd_name())
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

  pub fn subcmd_name() -> &'static str {
    "new"
  }
}

impl Command for New {
  fn execute<'a>(&self, args: &ArgMatches<'a>) {
    println!("{}", New::subcmd_name())
  }
}
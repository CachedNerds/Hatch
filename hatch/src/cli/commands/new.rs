use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

use manifest::Manifest;

use project::{ ProjectKind, LibraryKind };

use hatch_error::{
  HatchError,
  NullError
};

pub struct New {
  name: &'static str
}

impl New {
  pub fn new() -> New {
    New { name: "new" }
  }
}

impl<'command> Command<'command> for New {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Creates a new project. (default = shared library)")
      .version("0.1.0")

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

  fn execute(&self, args: &ArgMatches<'command>) -> Result<Manifest, HatchError> {
    println!("Project Path: {}\nToolbox Path: {}",
             self.project_name(args).unwrap(),
             self.project_path(args));
    Err(HatchError::Null(NullError))
  }
}

impl<'new> New {
  fn build_type(&self, args: &ArgMatches<'new>) -> ProjectKind {
    if args.is_present("bin") {
      ProjectKind::Binary
    } else if args.is_present("static") {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }
}

use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

use project::{ Project, ProjectKind, LibraryKind };

use std::fs;

// Must use qualified names to avoid conflict.
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

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

      .arg(Arg::with_name("PROJECT_NAME")
           .help("Name of project")
           .required(true).takes_value(true))

      .arg(Arg::with_name("bin")
           .help("Generate a stand alone executable")
           .long("bin").short("b").required(false)) 

      .arg(Arg::with_name("static")
           .help("Generate a static library")
           .long("static").short("s").conflicts_with("bin").required(false))

      .arg(Arg::with_name("VERSION")
           .help("Set the project version")
           .long("version").short("v").required(false).takes_value(true))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Result<Project, HatchError> {
    match fs::create_dir(self.project_path(args) + self.project_name(args).unwrap().as_str()) {
      Err(e) => Err(HatchError::from(e)),
      Ok(_) => {
        let mut yaml_output = String::new();
        
        write!(&mut yaml_output, "name: {}\nversion: {}\nbuild: {}\n",
               self.project_name(args).unwrap(),
               self.project_version(args),
               self.project_kind(args));

        match fs::File::create(self.project_path(args)
                               + self.project_name(args).unwrap().as_str()
                               + "/Hatch.yml") {
          Err(e) => Err(HatchError::from(e)),
          Ok(mut file) => {
            match file.write_all(yaml_output.as_bytes()) {
              Err(e) => Err(HatchError::from(e)),
              Ok(_) => Ok(Project::new(
                  self.project_name(args).unwrap(),
                  self.project_kind(args),
                  self.project_path(args),
                  self.project_version(args)))
            }
          }
        }
      }
    }
  }
}

impl<'new> New {
  fn project_version(&self, args: &ArgMatches<'new>) -> String {
    if args.is_present("VERSION") {
      value_t!(args, "VERSION", String).unwrap()
    } else {
      "0.0.1".to_owned()
    }
  }

  fn project_kind(&self, args: &ArgMatches<'new>) -> ProjectKind {
    if args.is_present("bin") {
      ProjectKind::Binary
    } else if args.is_present("static") {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }
}

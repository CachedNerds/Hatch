use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

use yaml;

use yaml_rust::Yaml;

use project::{ Project };

use hatch_error::{
  HatchError,
  MissingNameError,
  MissingBuildError,
  MissingVersionError,
  EmptyConfigError
};

pub struct Build {
  name: &'static str
}

impl<'build> Build {
  pub fn new() -> Build {
    Build { name: "build" }
  }

 }

impl<'command> Command<'command> for Build {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Builds a project.")
      .author("Josh Gould <mrgould93@gmail.com>")
      
      .arg(Arg::with_name("PROJECT_NAMES")
           .help("The projects to be built.")
           .required(false)
           .min_values(0)
           .value_delimiter(" "))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<Result<Project, HatchError>> {
    let projects_vec = self.project_names(args);
    let path = self.project_path(args);

    if projects_vec.len() == 0 {
      println!("Building all projects at path: {}", &path);
    } else {
      println!("Building {:?} at path: {}", &projects_vec, &path);
    }
    // should check for Hatch.yml files at all locations and fail out if
    // errors
    Vec::new()
    // generate tup files parameterized with the current project
  }
}

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

  fn project_names(&self, args: &ArgMatches<'build>) -> Vec<String> {
    if args.is_present("PROJECT_NAMES") {
      values_t!(args, "PROJECT_NAMES", String).unwrap()
    } else {
      Vec::new()
    }
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

  fn execute(&self, args: &ArgMatches<'command>) -> Result<Vec<Project>, HatchError> {
    let projects_vec = self.project_names(args);

    if projects_vec.len() == 0 {
      println!("Building all projects");
    } else {
      println!("Building {:?}", &projects_vec);
    }
    
    Ok(Vec::new())
    // generate tup files parameterized with the current project
  }
}

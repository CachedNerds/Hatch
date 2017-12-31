use std::fs;
use std::path::{ Path, PathBuf };

use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::{ Command };

use yaml;

use yaml_rust::Yaml;

use hatch_error::HatchError;

use project::{ Project, LibraryKind, ProjectKind };

trait ProjectUpdater {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<Result<Project, HatchError>>; 
}

struct AmbiguousUpdater;
struct ExplicitUpdater;

impl ProjectUpdater for AmbiguousUpdater {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<Result<Project, HatchError>> {
    match yaml::from_file(path.to_owned() + "Hatch.yml") {
      Err(e) => vec![Err(e)],
      Ok(yml_vec) => vec![yaml::parse(yml_vec)],
    }
  }
}

impl ProjectUpdater for ExplicitUpdater {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<Result<Project, HatchError>> {
    let yaml_result = project_names.into_iter().map(|p| {
      yaml::from_file(path.to_owned() + &p[..] + "/Hatch.yml")
    }).collect::<Vec<_>>();

    yaml_result.into_iter().map(|i| {
      match i {
        Err(e) => Err(e),
        Ok(yml_vec) => yaml::parse(yml_vec),
      }
    }).collect::<Vec<_>>()
  }
}

pub struct Update { name: &'static str }

impl<'update> Update {
  pub fn new() -> Update {
    Update {
      name: "update"
    }
  }
}

impl<'command> Command<'command> for Update {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Josh Gould <mrgould93@gmail.com>") 

      .arg(Arg::with_name("PROJECT_NAMES")
           .help("The projects to be updated.")
           .min_values(0).value_delimiter(" ")
           .required(false))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<Result<Project, HatchError>> {
    let mut updater: Box<ProjectUpdater>;

    if args.is_present("PROJECT_NAMES") {
      updater = Box::new(ExplicitUpdater)
    } else {
      updater = Box::new(AmbiguousUpdater)
    }

    updater.execute(self.project_path(args), self.project_names(args))
  }
}

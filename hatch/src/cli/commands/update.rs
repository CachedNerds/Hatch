use HatchResult;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use cli::commands::ops::ProjectOps;
use yaml;
use project::Project;
use hatch_error::HatchError;

struct AmbiguousUpdater;
struct ExplicitUpdater;

impl ProjectOps for AmbiguousUpdater {
  fn execute(&self, path: String, _: Vec<String>) -> Vec<HatchResult<Project>> {
    vec![yaml::parse_one(path)]
  }
}

impl ProjectOps for ExplicitUpdater {
  fn execute(&self, path: String, project_names: Vec<String>) -> Vec<HatchResult<Project>> {
    yaml::parse_many(path, project_names)
  }
}

pub struct Update {
  name: &'static str
}

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

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<HatchResult<Project>> {
    let mut updater: Box<ProjectOps>;

    if args.is_present("PROJECT_NAMES") {
      updater = Box::new(ExplicitUpdater)
    } else {
      updater = Box::new(AmbiguousUpdater)
    }

    updater.execute(self.project_path(args), self.project_names(args))
  }
}

pub mod new;
pub mod update;
pub mod build;

use project::Project;
use clap::{ ArgMatches, App };
use hatch_error::HatchError;

pub trait Command<'command> {
  fn cli_subcommand(&self) -> App<'command, 'command>;
  
  fn subcommand_name(&self) -> &'static str;

  fn execute(&self, args: &ArgMatches<'command>) -> Vec<Result<Project, HatchError>>;
 
  fn project_names(&self, args: &ArgMatches<'command>) -> Vec<String> {
    if args.is_present("PROJECT_NAMES") {
      values_t!(args, "PROJECT_NAMES", String).unwrap()
    } else {
      Vec::new()
    }
  }
  
  fn project_path(&self, args: &ArgMatches<'command>) -> String {
    let mut path = String::new();

    if args.is_present("PROJECT_PATH") {
      path.push_str(value_t!(args, "PROJECT_PATH", String).unwrap().as_str());
    } else {
      path.push_str("./");
    }

    match path.as_str().chars().last().unwrap() {
      '/' => path,
      _   => {
        path.push_str("/");
        path
      }
    }
  }
}

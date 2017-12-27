pub mod new;
pub mod update;

use project::Project;
use clap::{ ArgMatches, App };
use hatch_error::HatchError;

pub trait Command<'command> {
  fn cli_subcommand(&self) -> App<'command, 'command>;
  
  fn subcommand_name(&self) -> &'static str;

  fn execute(&self, args: &ArgMatches<'command>) -> Result<Vec<Project>, HatchError>;

  fn project_name(&self, args: &ArgMatches<'command>) -> Option<String> {
    if args.is_present("PROJECT_NAME") {
      Some(value_t!(args, "PROJECT_NAME", String).unwrap())
    } else {
      None
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

use hatch_error::HatchResult;

pub mod new;
pub mod update;
pub mod build;

use std::path::PathBuf;

use project::Project;
use clap::{ ArgMatches, App };

static INCLUDE: &str = "INCLUDE";
static VERSION: &str = "VERSION";
static BIN: &str = "BIN";
static STATIC: &str = "STATIC";
static PROJECT_NAMES: &str = "PROJECT_NAMES";
static PROJECT_PATH: &str = "PROJECT_PATH";

pub trait Command<'command> {
  fn cli_subcommand(&self) -> App<'command, 'command>;
  
  fn subcommand_name(&self) -> &'static str;

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<Project>;
 
  fn project_name(&self, args: &ArgMatches<'command>) -> Option<String> {
    value_t!(args, PROJECT_NAMES, String).ok()
  }
  
  fn project_path(&self, args: &ArgMatches<'command>) -> PathBuf {
    if args.is_present(PROJECT_PATH) {
      PathBuf::from(value_t!(args, PROJECT_PATH, String).unwrap().as_str())
    } else {
      PathBuf::from("./")
    }
  }
}

fn parse_deps_from_cli<'func>(args: &ArgMatches<'func>) -> Vec<String> {
  if let Some(values) = args.values_of(INCLUDE) {
    values.map(String::from).collect::<Vec<String>>()
  } else {
    Vec::new()
  }
}

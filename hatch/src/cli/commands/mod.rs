pub mod new;
pub mod update;
pub mod build;
pub mod test;
pub mod run;

use hatch_error::HatchResult;
use clap::{ ArgMatches, App };
use std::path::PathBuf;
use deps::dependency::Dependency;

static ARGS: &str = "ARGS";
static INCLUDE: &str = "INCLUDE";
static VERSION: &str = "VERSION";
static TYPE: &str = "TYPE";

pub static BIN: &str = "bin";
pub static STATIC: &str = "static";
pub static SHARED: &str = "shared";
pub static HEADER: &str = "header-only";

static PROJECT_NAME: &str = "PROJECT_NAME";
static PROJECT_NAMES: &str = "PROJECT_NAMES";
static PROJECT_PATH: &str = "PROJECT_PATH";

pub trait Command<'command> {
  fn cli_subcommand(&self) -> App<'command, 'command>;
  
  fn subcommand_name(&self) -> &'static str;

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()>;
 
  fn project_name(&self, args: &ArgMatches<'command>) -> Option<String> {
    value_t!(args, PROJECT_NAME, String).ok()
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

fn parse_dependencies<'func>(args: &ArgMatches<'func>) -> Vec<Dependency> {
  if let Some(values) = args.values_of(INCLUDE) {
    values.map(String::from).map(Dependency::new).collect::<Vec<Dependency>>()
  } else {
    Vec::new()
  }
}
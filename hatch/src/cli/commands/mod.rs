use hatch_error::HatchResult;

pub mod new;
pub mod update;
pub mod build;
pub mod test;

use clap::{ ArgMatches, App };

static ARGS: &str = "ARGS";
static INCLUDE: &str = "INCLUDE";
static VERSION: &str = "VERSION";
static BIN: &str = "BIN";
static STATIC: &str = "STATIC";
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
  
  fn project_path(&self, args: &ArgMatches<'command>) -> String {
    let mut path = String::new();

    if args.is_present(PROJECT_PATH) {
      path.push_str(value_t!(args, PROJECT_PATH, String).unwrap().as_str());
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

fn parse_deps_from_cli<'func>(args: &ArgMatches<'func>) -> Vec<(String, String)> {
  let mut parsed_deps = Vec::new();
  if let Some(values) = args.values_of(INCLUDE) {
    let mut vals = values.map(String::from).collect::<Vec<String>>().into_iter();
    while vals.len() != 0 {
      let url = vals.next();
      let name = vals.next();
      parsed_deps.push((url.unwrap(), name.unwrap()));
    }
  }
  parsed_deps
}

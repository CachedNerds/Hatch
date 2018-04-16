pub mod new;
pub mod update;
pub mod build;
pub mod test;
pub mod run;

use hatch_error::HatchResult;
use clap::{ ArgMatches };
use std::path::PathBuf;
use deps::dependency::Dependency;
use project::ProjectKind;
use constants::{ ARGS, PROJECT_NAME, PROJECT_PATH, VERSION, TYPE, INCLUDE };
use std::path::Path;
use platform::os;
use assets::PlatformKind;
use std::process;
use hatch_error::InvalidPathError;

pub trait Command<'command> {
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

  fn project_version(&self, args: &ArgMatches<'command>) -> String {
    if args.is_present(VERSION) {
      value_t!(args, VERSION, String).unwrap()
    } else {
      "0.0.1".to_owned()
    }
  }

  fn project_kind(&self, args: &ArgMatches<'command>) -> ProjectKind {
    if args.is_present(TYPE) {
      let type_arg: String = value_t!(args, TYPE, String).unwrap();
      ProjectKind::from_str(type_arg.as_str())
    } else {
      ProjectKind::default()
    }
  }

  fn parse_arguments_from_cli(&self, cli_args: &ArgMatches<'command>) -> Vec<String> {
    if let Some(arguments) = cli_args.values_of(ARGS) {
      arguments.map(String::from).collect()
    } else {
      Vec::new()
    }
  }

  fn build(&self, project_path: &Path) -> HatchResult<()> {
    if let Some(path) = project_path.to_str() {
      let command = format!("cd {} && tup", path);
      let mut shell: String;
      let mut args: Vec<String>;
      match os::platform_type() {
        PlatformKind::Windows => {
          shell = String::from("cmd");
          args = vec![String::from("/C"), command];
        },
        _ => {
          shell = String::from("sh");
          args = vec![String::from("-c"), command];
        }
      }

      let mut child = process::Command::new(shell).args(args).spawn()?;
      child.wait()?;

      Ok(())
    } else {
      Err(InvalidPathError)?
    }
  }
}

fn parse_dependencies<'func>(args: &ArgMatches<'func>) -> Vec<Dependency> {
  if let Some(values) = args.values_of(INCLUDE) {
    values.map(String::from).map(Dependency::new).collect::<Vec<Dependency>>()
  } else {
    Vec::new()
  }
}

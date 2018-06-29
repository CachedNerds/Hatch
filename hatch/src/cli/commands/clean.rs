use hatch_error::{ HatchResult, CleanupError };
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use project::Project;
use generators::tup::make_a_tup_in_a_box;

pub struct Clean {
  name: &'static str
}

impl<'update> Clean {
  pub fn new() -> Clean {
    Clean {
      name: "clean"
    }
  }

  fn print_errors(&self, results: &Vec<HatchResult<()>>) {
    for result in results {
      match result.as_ref() {
        Ok(_) => {},
        Err(ref e) => {
          println!("{}", e);
        }
      }
    }
  }

  // determine if the collection of results contains an error
  fn contains_errors(&self, results: &Vec<HatchResult<()>>) -> bool {
    for result in results {
      match result.as_ref() {
        Ok(_) => {},
        Err(_) => {
          return true;
        }
      }
    }

    return false;
  }

  pub fn clean(&self, project: &Project) -> HatchResult<()> {
    let generator = make_a_tup_in_a_box();
      let clear_result = generator.clear_assets();
      // TODO: get back printing errors
      match clear_result {
          Ok(()) => Ok(()),
          Err(e) => CleanupError,
      }
  }
}

impl<'command> Command<'command> for Clean {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
      let (_, project) = self.read_project_context(&args)?;
    self.clean(&project)?;
    Ok(())
  }
}

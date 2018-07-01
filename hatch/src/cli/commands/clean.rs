use hatch_error::{ HatchResult, CleanupError };
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use project::Project;
use generators::tup::make_a_tup_in_a_box;
use std::path::PathBuf;

pub struct Clean {
  name: &'static str
}

impl<'clean> Clean {
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

  pub fn clean(&self, args: &ArgMatches<'clean>) -> HatchResult<()> {
      let (project_path, project) = self.read_project_context(&args)?;
      let generator = make_a_tup_in_a_box();
      let clear_result = generator.clear_assets(project_path, &project);
      let mut errors = false;
      for result in clear_result {
          if let Err(e) = result {
              println!("{}", e);
              errors = true;
          }
      }
      if errors { Ok(()) } else { Err(CleanupError)? }
  }
}

impl<'command> Command<'command> for Clean {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    self.clean(args)?;
    Ok(())
  }
}

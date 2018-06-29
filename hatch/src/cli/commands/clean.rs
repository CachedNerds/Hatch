use hatch_error::{ HatchResult, CleanupError };
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use project::Project;
use assets::builder::Builder as AssetBuilder;
use janitor;

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
    let asset_builder = AssetBuilder::from(&project);

    let remove_assets_results = janitor::remove_assets(asset_builder.assets());
    let remove_targets_results = janitor::remove_targets(project.path());

    self.print_errors(&remove_assets_results);
    self.print_errors(&remove_targets_results);

    let errors_occurred = self.contains_errors(&remove_assets_results) || self.contains_errors(&remove_targets_results);
    if errors_occurred {
      Err(CleanupError)?
    } else {
      Ok(())
    }
  }
}

impl<'command> Command<'command> for Clean {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project = task::read_project(&self.project_path(args))?;
    self.clean(&project)?;
    Ok(())
  }
}

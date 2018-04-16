use hatch_error::HatchResult;
use clap::{ ArgMatches };
use cli::commands::Command;
use task;
use generators::tup::Tup;

pub struct Update;

impl<'update> Update {
  pub fn new() -> Update {
    Update
  }
}

impl<'command> Command<'command> for Update {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project = task::read_project(&self.project_path(args))?;
    let generator = Tup::boxed(&project);
    task::generate_assets(generator, &project)?;
    Ok(())
  }
}

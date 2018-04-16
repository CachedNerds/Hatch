use hatch_error::HatchResult;
use clap::{ ArgMatches };
use cli::commands::Command;
use task;

pub struct Update;

impl<'update> Update {
  pub fn new() -> Update {
    Update
  }
}

impl<'command> Command<'command> for Update {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project = task::read_project(&self.project_path(args))?;

    task::generate_assets(&project)?;

    Ok(())
  }
}

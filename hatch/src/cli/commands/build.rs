use hatch_error::{ HatchResult };
use clap::{ ArgMatches };
use cli::commands::Command;
use task;
use generators::tup::Tup;

pub struct Build;

impl<'build> Build {
  pub fn new() -> Build {
    Build
  }
}

impl<'command> Command<'command> for Build {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project_path = self.project_path(args);
    let project = task::read_project(&project_path)?;
    println!("Generating assets...\n");
    let generator = Tup::boxed(&project);
    task::generate_assets(generator, &project)?;
    println!("Building project...\n");
    self.build(&project_path)?;
    Ok(())
  }
}

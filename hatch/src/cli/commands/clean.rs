use hatch_error::HatchResult;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use task;
use project::Project;
use assets::builder::Builder as AssetBuilder;
use assets::janitor;

pub struct Clean {
  name: &'static str
}

impl<'update> Clean {
  pub fn new() -> Clean {
    Clean {
      name: "clean"
    }
  }
}

impl<'command> Command<'command> for Clean {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Removes volatile project artifacts.")
      .version("0.1.0")
      .author("Danny Peck <danieljpeck93@gmail.com>")
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project = task::read_project(&self.project_path(args))?;
    let asset_builder = AssetBuilder::from(&project);
    janitor::remove_assets(asset_builder.assets());
    janitor::remove_targets(project.path());

    Ok(())
  }
}

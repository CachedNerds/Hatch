extern crate hatch;

use std::collections::HashMap;

use hatch::manifest::Manifest;
use hatch::project::Project;
use hatch::asset::Asset;
use hatch::cli::Cli;
use hatch::cli::commands::Command;
use hatch::cli::commands::new::New;
use hatch::cli::commands::update::Update;
use hatch::clap::ArgMatches;

fn main() {
  // create the subcommand to command map
  let mut commands: HashMap<&'static str, Box<Command>> = HashMap::new();
  commands.insert(New::subcmd_name(), Box::new(New));
  commands.insert(Update::subcmd_name(), Box::new(Update));

  // create list of subcmd clap objects
  let subcommands = vec![New::subcmd(), Update::subcmd()];

  let cli = Cli::new(subcommands);
  match cli.subcommand() {
    (subcommand, Some(args)) => {
      match commands.get(subcommand) {
        Some(command) => command.execute(args),
        _ => {}
      }
    },
    _ => {}
  };
}
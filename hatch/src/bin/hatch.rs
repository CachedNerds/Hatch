extern crate hatch;

use std::collections::HashMap;

use hatch::cli::Cli;
use hatch::cli::commands::Command;
use hatch::cli::commands::new::New;
use hatch::cli::commands::update::Update;
use hatch::cli::commands::build::Build;

use hatch::hatch_error::{ HatchError, NullError };

fn main() {
  // create the subcommand to command map
  let mut subcommands: HashMap<&'static str, Box<Command>> = HashMap::new();

  let new_command = Box::new(New::new());
  subcommands.insert(new_command.subcommand_name(), new_command);

  let update_command = Box::new(Update::new());
  subcommands.insert(update_command.subcommand_name(), update_command);

  let build_command = Box::new(Build::new());
  subcommands.insert(build_command.subcommand_name(), build_command);

  // initialize cli with the set of subcommand
  let cli = Cli::new(subcommands.values().map(|v| v.cli_subcommand()).collect::<Vec<_>>());

  // execute selected subcommand
  match cli.subcommand() {
    (subcommand_name, Some(cli_args)) => {
      match subcommands.get(subcommand_name) {
        Some(subcommand) => subcommand.execute(cli_args),
        _ => vec![Err(HatchError::Null(NullError))]
      }
    },
    _ => vec![Err(HatchError::Null(NullError))]
  };
}

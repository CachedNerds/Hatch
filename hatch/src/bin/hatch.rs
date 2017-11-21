extern crate hatch;

use std::collections::HashMap;

use hatch::cli::Cli;
use hatch::cli::commands::Command;
use hatch::cli::commands::new::New;
use hatch::cli::commands::update::Update;

fn main() {
  // create the subcommand to command map
  let mut subcommands: HashMap<&'static str, Box<Command>> = HashMap::new();

  let new_command = Box::new(New::new());
  subcommands.insert(new_command.subcommand_name(), new_command);

  let update_command = Box::new(Update::new());
  subcommands.insert(update_command.subcommand_name(), update_command);

  // create list of cli subcommand options
  let mut cli_subcommands = Vec::new();
  for (_subcommand_name, subcommand) in &subcommands {
    cli_subcommands.push(subcommand.cli_subcommand());
  }
  
  // initialize cli with the set of subcommands
  let cli = Cli::new(cli_subcommands);

  // execute selected subcommand
  match cli.subcommand() {
    (subcommand_name, Some(cli_args)) => {
      match subcommands.get(subcommand_name) {
        Some(subcommand) => subcommand.execute(cli_args),
        _ => {}
      }
    },
    _ => {}
  };
}
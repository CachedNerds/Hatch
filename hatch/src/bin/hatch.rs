extern crate hatch;
extern crate clap;
extern crate yaml_rust;

use std::collections::HashMap;

use hatch::cli::commands::Command;
use hatch::cli::commands::run::Run;
use clap::App;
use yaml_rust::YamlLoader;
use hatch::hatch_error::{ HatchResult, MissingParameterError };
use hatch::cli::commands::new::New;
use hatch::cli::commands::update::Update;
use hatch::cli::commands::build::Build;
use hatch::cli::commands::test::Test;
use hatch::constants;

fn do_me_a_hatch() -> HatchResult<()> {
  let cli = include_str!("cli.yml");
  let docs = YamlLoader::load_from_str(cli)?;
  let doc = &docs[0];
  let matches = App::from_yaml(doc).get_matches();
  let (name, args) = matches.subcommand();

  let mut subcommands: HashMap<&'static str, Box<Command>> = HashMap::new();
  let run_command = Box::new(Run::new());
  subcommands.insert(constants::RUN_NAME, run_command);

  let new_command = Box::new(New::new());
  subcommands.insert(constants::NEW_NAME, new_command);

  let update_command = Box::new(Update::new());
  subcommands.insert(constants::UPDATE_NAME, update_command);

  let build_command = Box::new(Build::new());
  subcommands.insert(constants::BUILD_NAME, build_command);

  let test_command = Box::new(Test::new());
  subcommands.insert(constants::TEST_NAME, test_command);

  let subcommand = subcommands.get(name).ok_or_else(|| MissingParameterError)?;

  // TODO: args.unwrap is bad, don't do this, it should be resultified
  subcommand.execute(args.unwrap())?;
  Ok(())
}

fn main() {

  println!("running hatch...");

  match do_me_a_hatch() {
    Ok(()) => {
      println!("done");
      ()
    },
    Err(e) => {
      println!("error: {:?}", e);
      ()
    }
  }
}

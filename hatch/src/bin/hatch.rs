extern crate hatch;
extern crate clap;
extern crate yaml_rust;

use std::collections::HashMap;

use hatch::cli::commands::Command;
use hatch::cli::commands::run::Run;
use clap::App;
use yaml_rust::YamlLoader;
use hatch::hatch_error::HatchResult;
use hatch::hatch_error::MissingParameterError;
use hatch::cli::commands::new::New;
use hatch::cli::commands::update::Update;
use hatch::cli::commands::build::Build;
use hatch::cli::commands::test::Test;

fn do_me_a_hatch() -> HatchResult<()> {
  let cli = include_str!("cli.yml");
  let docs = YamlLoader::load_from_str(cli)?;
  let doc = &docs[0];
  let matches = App::from_yaml(doc).get_matches();
  let (name, args) = matches.subcommand();

  let mut subcommands: HashMap<&'static str, Box<Command>> = HashMap::new();
  let run_command = Box::new(Run::new());
  subcommands.insert(run_command.subcommand_name(), run_command);

  let new_command = Box::new(New::new());
  subcommands.insert(new_command.subcommand_name(), new_command);

  let update_command = Box::new(Update::new());
  subcommands.insert(update_command.subcommand_name(), update_command);

  let build_command = Box::new(Build::new());
  subcommands.insert(build_command.subcommand_name(), build_command);

  let test_command = Box::new(Test::new());
  subcommands.insert(test_command.subcommand_name(), test_command);

  let subcommand = subcommands.get(name).ok_or_else(|| MissingParameterError)?;
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

  // create the subcommand to command map

//  let new_command = Box::new(New::new());
//  subcommands.insert(new_command.subcommand_name(), new_command);

//  let update_command = Box::new(Update::new());
//  subcommands.insert(update_command.subcommand_name(), update_command);
//
//  let build_command = Box::new(Build::new());
//  subcommands.insert(build_command.subcommand_name(), build_command);
//
//  let test_command = Box::new(Test::new());
//  subcommands.insert(test_command.subcommand_name(), test_command);
//
  //let run_command = Box::new(Run::new());
  //subcommands.insert(run_command.subcommand_name(), run_command);


  //let subcommand = subcommands.get(name).ok_or_else(|| 0)?;
  //let subcommand = subcommands.get(name).unwrap();
  //let res = subcommand.execute(args.unwrap());


//  if let Some(cmd) = subcommands.get(name) {
//    if let Err(e) = cmd.execute(args.unwrap()) {
//      println!("{}", e);
//    }
//  }
  //let deserialized_point: Point = serde_yaml::from_str(&s).unwrap();
//  let docs = YamlLoader::load_from_str(cli).unwrap();
//  let doc = &docs[0];
//  let matches = App::from_yaml(doc).get_matches();
//  let (name, args) = matches.subcommand();
//  if let Some(cmd) = subcommands.get(name) {
//    if let Err(e) = cmd.execute(args.unwrap()) {
//      println!("{}", e);
//    }
//  }
//  create the subcommand to command map
//  let mut subcommands: HashMap<&'static str, Box<Command>> = HashMap::new();
//
//  let new_command = Box::new(New::new());
//  subcommands.insert(new_command.subcommand_name(), new_command);
//
//  let update_command = Box::new(Update::new());
//  subcommands.insert(update_command.subcommand_name(), update_command);
//
//  let build_command = Box::new(Build::new());
//  subcommands.insert(build_command.subcommand_name(), build_command);
//
//  let test_command = Box::new(Test::new());
//  subcommands.insert(test_command.subcommand_name(), test_command);
//
//  let run_command = Box::new(Run::new());
//  subcommands.insert(run_command.subcommand_name(), run_command);
//
//  // initialize cli with the set of subcommand
//  let cli = Cli::new(subcommands.values().map(|v| v.cli_subcommand()).collect::<Vec<_>>());
//
//  // execute selected subcommand
//  let (name, args) = cli.subcommand();
//  if let Some(cmd) = subcommands.get(name) {
//    if let Err(e) = cmd.execute(args.unwrap()) {
//      println!("{}", e);
//    }
//  }
}

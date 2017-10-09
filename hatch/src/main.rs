use std::fs;
use std::path::{ PathBuf };
use std::io::{ Read, ErrorKind };

#[macro_use]
extern crate clap;

mod cli;
mod project;
mod errors;

use project::{ Project, RunKind };
use project::ProjectType::{ Binary, Library };
use project::LibraryType::{ Shared, Static };
use project::metadata::{ Metadata };

fn main() {
  let result = match cli::build_cli()
    .get_matches()
    .subcommand() {

      ("new", Some(args)) => create_new_project(args),
      ("update", Some(args)) => update_existing_project(args),
      // We will never execute this branch
      _ => Err(errors::Error::NullError),
    };

  match result {
    Ok(project) =>
      if let Err(e) = project.manage_files() {
        println!("Error: {}", e);
      },
    Err(e) =>
      println!("Error: {}", e),
  }

}

fn get_name(args: &clap::ArgMatches) -> Result<String, errors::Error> {
  Ok(value_t!(args, "PROJECT_NAME", String)?)
}

fn create_new_project(args: &clap::ArgMatches) -> Result<Project, errors::Error> {
  let name = get_name(&args)?;
  let mut metadata = Metadata::new(&args)?; 
  
  metadata.path.push(name.clone());

  let build_type = if args.is_present("bin") {
    Binary
  } else if args.is_present("static") {
    Library(Static)
  } else {
    Library(Shared)
  };

  Ok(Project { name, build_type, metadata, run_type: RunKind::Create })
}

fn update_existing_project(args: &clap::ArgMatches) -> Result<Project, errors::Error> {
  let name = get_name(args)?;
  let mut metadata = Metadata::new(&args)?; 
  let mut build_type = String::new();

  metadata.path.push(name.clone());
  metadata.path.push("config.tup");

  let _ = fs::File::open(&metadata.path)
    .and_then(|mut file| file.read_to_string(&mut build_type))?;

  let _ = metadata.path.pop();

  build_type = build_type
    .lines()
    .filter(|line| line.contains("LIB_TYPE"))
    .collect();
  
  let build_type = match &*build_type {
    "LIB_TYPE = static"  => Library(Static), 
    "LIB_TYPE = shared"  => Library(Shared),
    _                    => Binary,
  };

  Ok(Project { name, build_type, metadata, run_type: RunKind::Update })
}

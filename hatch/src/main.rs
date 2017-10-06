use std::fs;
use std::path::{ PathBuf };
use std::io::{ Read };

#[macro_use]
extern crate clap;
use clap::ArgMatches;

mod cli;
mod project;
mod error;

use project::{ Project, Command };
use project::ProjectType::{ Binary, Library };
use project::LibraryType::{ Shared, Static };

fn main() {
  let result = match cli::build_cli()
    .get_matches()
    .subcommand() {

      ("new", Some(args)) => create_new_project(args),
      ("update", Some(args)) => update_existing_project(args),
      // We will never execute this branch
      _ => Err(error::Error::from("Invalid version")),
    };


  match result {
    Ok(project) => project.execute(),
    Err(e) => println!("Error: {}", e),
  }
}

fn toolbox_path(args: &ArgMatches) -> Result<PathBuf, error::Error> {
  let mut path = PathBuf::new();
  
  if args.is_present("TOOLBOX_PATH") {
    path.push(value_t!(args, "TOOLBOX_PATH", String)?);
  } else {
    path.push("./");
  }

  Ok(path)
}

fn get_version(args: &ArgMatches) -> Result<(u16, u16, u16), error::Error> {
  match values_t!(args, "PROJECT_VERSION", u16) {
    Ok(v) => {
      if v.iter().count() == 3 { Ok((v[0], v[1], v[2])) }
      else {
        Err(error::Error::from("Invalid version"))
      }
    },
    Err(_) => Ok((0, 0, 1))
  }
}

fn create_new_project(args: &ArgMatches) -> Result<Project, error::Error> {
  let project_path = toolbox_path(args)?;

  let project_name = value_t!(args, "PROJECT_NAME", String)?;
  
  let project_type = if args.is_present("bin") {
    Binary
  } else if args.is_present("static") {
    Library(Static)
  } else {
    Library(Shared)
  };

  let project_version = get_version(args)?;

  Ok(Project {
    project_name,
    project_type,
    project_version,
    project_path })
}

fn update_existing_project(args: &ArgMatches) -> Result<Project, error::Error> {
  let mut project_path = toolbox_path(args)?;

  let project_name = value_t!(args, "PROJECT_NAME", String)?;

  project_path.push("C++/libs");
  project_path.push(project_name.clone());

  let mut _project_type = String::new();

  project_path.push("config.tup");

  let _ = fs::File::open(&project_path)
    .and_then(|mut file| file.read_to_string(&mut _project_type))?;

  let _ = project_path.pop();

  _project_type = _project_type
    .lines()
    .filter(|line| line.contains("LIB_TYPE"))
    .collect();
  
  let project_type = match _project_type.split_whitespace().last().unwrap_or("") {
    "static"  => Library(Static), 
    "shared"  => Library(Shared),
    _         => Binary,
  };

  Ok(Project {
    project_name,
    project_type,
    project_version: (0, 0, 1),
    project_path })
}

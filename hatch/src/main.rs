use std::fs;
use std::path::{ PathBuf };

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

  let mut dirs: Vec<fs::DirEntry> = Vec::new();
  let mut files: Vec<fs::DirEntry> = Vec::new();

  // search for `config.tup`
  //  find it ? read contents : throw error

  for path in fs::read_dir(&project_path)? {
    if path.as_ref().unwrap().file_type()?.is_dir() {
      dirs.push(path?)
    } else {
      files.push(path?)
    }
  }
  
  Ok(Project {
    project_name,
    project_type: Binary,
    project_version: (0, 0, 1),
    project_path })
}

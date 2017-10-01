use std::fs;

#[macro_use]
extern crate clap;
use clap::ArgMatches;

mod cli;
mod error;
mod project;

use error::{ Error };
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
      _ => Err(Error::NullError),
    };


  match result {
    Ok(project) => project.execute(),
    Err(e) => println!("{:?}", e),
  }
}

fn create_new_project(args: &ArgMatches) -> Result<Project, Error> {
  let project_name = value_t!(args, "PROJECT_NAME", String)?;
  
  let project_type = if args.is_present("bin") {
    Binary
  } else if args.is_present("static") {
    Library(Static)
  } else {
    Library(Shared)
  };
  
  let project_version = (0, 0, 1);

  Ok(Project {
    project_name,
    project_type,
    project_version })
}

fn update_existing_project(args: &ArgMatches) -> Result<Project, Error> {
  let mut dirs: Vec<fs::DirEntry> = Vec::new();
  let mut files: Vec<fs::DirEntry> = Vec::new();

  // cause error to be thrown here by passing in path from args
  for path in fs::read_dir("./")? {
    if path.as_ref().unwrap().file_type()?.is_dir() {
      dirs.push(path?)
    } else {
      files.push(path?)
    }
  }

  println!("Dirs: {:?}\nFiles: {:?}", dirs, files);
  
  Ok(Project {
    project_name: "foo".to_string(),
    project_type: Binary,
    project_version: (0, 0, 1) })
}

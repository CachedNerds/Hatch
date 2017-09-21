use std::fs;

#[macro_use]
extern crate clap;
use clap::ArgMatches;

mod cli;
mod error;
mod project;

use error::{ ErrorT };
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
      _ => Err(ErrorT::NullError),
    };


  match result {
    Ok(project) => project.execute(),
    Err(e) => println!("{:?}", e),
  }
}

fn create_new_project<'a>(args: &ArgMatches) -> Result<Project, ErrorT> {
  let project_name = value_t!(args, "PROJECT_NAME", String)?;
  
  let project_type = match args.is_present("bin") {
    true  => Binary,
    false => {
      match args.is_present("static") {
        true  => Library(Static),
        false => Library(Shared),
      }
    }
  };
  
  let project_version = (0, 0, 1);

  Ok(Project {
    project_name,
    project_type,
    project_version })
}

fn update_existing_project<'a>(args: &ArgMatches) -> Result<Project, ErrorT> {
  let paths = fs::read_dir("./")?;

  let mut dirs: Vec<fs::DirEntry> = Vec::new();
  let mut files: Vec<fs::DirEntry> = Vec::new();

  for path in paths {
    if path.as_ref().unwrap().file_type().unwrap().is_dir() {
      dirs.push(path.unwrap())
    } else {
      files.push(path.unwrap())
    }
  }

  println!("Dirs: {:?}\nFiles: {:?}", dirs, files);
  // read in existing fs structure
  // create project instance to reflect existing project
  // return project
  Ok(Project {
    project_name: "foo".to_string(),
    project_type: Binary,
    project_version: (0, 0, 1) })
}

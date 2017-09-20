use std::fs;

#[macro_use]
extern crate clap;
use clap::ArgMatches;

mod cli;
mod project;

use project::Project;
use project::ProjectType::{ Binary, Library };
use project::LibraryType::{ Shared, Static };

fn main() {
  let result = match cli::build_cli()
    .get_matches()
    .subcommand() {

      ("new", Some(args)) => create_new_project(args),
      ("update", Some(args)) => update_existing_project(args),
      // We will never execute this branch
      _ => Err("")
    };


  match result {
    Ok(project) => println!("{:?}", project),
    Err(e) => println!("{}", e),
  }
}

fn create_new_project<'a>(args: &ArgMatches) -> Result<Project, &'a str> {
  let project_name = value_t!(args, "PROJECT_NAME", String).unwrap();
  let project_type =  
    if args.is_present("bin") { Binary }
    else {
      match args.is_present("static") {
        true => Library(Static),
        false => Library(Shared),
      }
    };
  let project_version = (0, 0, 1);

  Ok(Project {
    project_name,
    project_type,
    project_version })
}

fn entries_in_cwd() -> std::io::Result<fs::ReadDir> {
  fs::read_dir("./")
}

fn update_existing_project<'a>(args: &ArgMatches) -> Result<Project, &'a str> {
  let files = entries_in_cwd()
    .unwrap()
    .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file());
 
  // search for tupfiles in the files and handle appropriately

  let dirs = entries_in_cwd()
    .unwrap()
    .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_dir());

  // read in existing fs structure
  // create project instance to reflect existing project
  // return project
  Ok(Project {
    project_name: "foo".to_string(),
    project_type: Binary,
    project_version: (0, 0, 1) })
}

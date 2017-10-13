use std::fs;
use std::path;
use std::io::{ Read };

use tup::{ Project, LibraryKind, ProjectKind };
use tup::{ Manifest };

#[macro_use]
extern crate clap;

mod cli;
mod tup;
mod errors;

fn main() {
  let result = match cli::build_cli()
    .get_matches()
    .subcommand() {

      ("new", Some(args)) => create_new_project(args),
      ("update", Some(args)) => update_existing_project(args),
      // We will never execute this branch
      _ => Err(errors::Error::NullError),
    };

  match &result {
    &Ok(ref project) => {
      println!("{:?}", project);

    },
    &Err(ref e) => println!("Error: {}", e),
  }
}

fn path(args: &clap::ArgMatches) -> Result<path::PathBuf, errors::Error> {
  let mut path = path::PathBuf::new();
  
  if args.is_present("TOOLBOX_PATH") {
    path.push(value_t!(args, "TOOLBOX_PATH", String)?);
  } else {
    path.push("./");
  }
  
  path.push("C++/libs");

  Ok(path)
}

fn name(args: &clap::ArgMatches) -> Result<String, errors::Error> {
  Ok(value_t!(args, "PROJECT_NAME", String)?)
}

fn create_new_project(args: &clap::ArgMatches) -> Result<Project, errors::Error> {
  let name = name(&args)?;
  
  let path = path(&args).and_then(|mut p| {
    p.push(name.clone());
    Ok(p)
  })?;

  let build_type = if args.is_present("bin") {
    ProjectKind::Binary
  } else if args.is_present("static") {
    ProjectKind::Library(LibraryKind::StaticLib)
  } else {
    ProjectKind::Library(LibraryKind::SharedLib)
  };

  Ok(Project::new(name, build_type, path))
}

fn update_existing_project(args: &clap::ArgMatches) -> Result<Project, errors::Error> {
  let name = name(args)?;

  let mut build_type = String::new();
  
  let path = path(&args).and_then(|mut p| {
    p.push(name.clone());
    p.push("config.tup");
    
    let _ = fs::File::open(&p)
      .and_then(|mut file| file.read_to_string(&mut build_type))?;

    let _ = p.pop();

    Ok(p)
  })?;

  build_type = build_type
    .lines()
    .filter(|line| line.contains("LIB_TYPE"))
    .collect();

  let build_type = ProjectKind::from_str(build_type.split(' ').last().unwrap_or(""));
  
  Ok(Project::new(name, build_type, path))
}

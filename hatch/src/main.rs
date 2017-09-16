#[macro_use]
extern crate clap;
use clap::ArgMatches;

mod cli;
mod project;

use project::Project;
use project::ProjectType::{ Binary, Library };
use project::LibraryType::{ Shared, Static };

fn main() {
  let project = match cli::build_cli()
    .get_matches()
    .subcommand() {

      ("new", Some(args)) => create_new_project(args),
      _ => help(),
    };


  match project {
    Ok(p) => println!("{:?}", p),
    Err(e) => println!("{}", e),
  }
}

fn help<'a>() -> Result<Project, &'a str> {
  Err("No arguments were given")
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

  Ok(Project { project_name, project_type })
}

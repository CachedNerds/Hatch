extern crate clap;
use clap::{Arg, App, SubCommand};

pub struct Project {
  pub name: String,
  pub project_type: ProjectType,
}

pub enum LibraryType {
  Shared,
  Static,
}

pub enum ProjectType {
  Binary,
  Library(LibraryType),
}

impl Project {
  pub fn new_project(args: &clap::ArgMatches) -> Project {
    let project_name = value_t!(args, "PROJECT_NAME", String).unwrap();

    let project_type = if args.is_present("bin") {
      ProjectType::Binary
    } else {
      if args.is_present("static") {
        ProjectType::Library(LibraryType::Static)
      } else {
        ProjectType::Library(LibraryType::Shared)
      }
    };

    Project {
      name: project_name,
      project_type,
    }
  }
}

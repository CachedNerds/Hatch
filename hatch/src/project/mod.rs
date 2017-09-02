extern crate clap;

pub struct Project {
  pub project_name: String,
  pub project_type: ProjectType,
}

#[derive(Debug)]
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
      project_name,
      project_type,
    }
  }
}

// TODO:
// Implement trait that ProjectType & LibraryType conform to
// such that we can act upon any kind of project be it a
// binary or a library shared or static

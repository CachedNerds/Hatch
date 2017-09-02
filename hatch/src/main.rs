#[macro_use]
extern crate clap;

mod cli;
mod project;

use project::Project;
use project::ProjectType;
use project::LibraryType;

fn main() {
  match cli::build_cli().get_matches().subcommand() {
    ("new", Some(args)) => create_new_project(Project::new_project(args)),
    _ => println!("under construction"),
  }
}

fn create_new_project(project: Project) {
  match project.project_type {
    ProjectType::Binary => println!("binary"),
    ProjectType::Library(lib_type) => println!("{:?}", lib_type),
  }
}

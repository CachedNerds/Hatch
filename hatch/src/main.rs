#[macro_use]
extern crate clap;

mod cli;
mod project;

use project::Project;

fn main() {
  let project: Project;

  match cli::build_cli().get_matches().subcommand() {
    ("new", Some(args)) => project = Project::new_project(args),
    _ => println!("under construction"),
  }
}

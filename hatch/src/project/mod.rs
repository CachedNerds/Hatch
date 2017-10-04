use std::path::{ PathBuf };

#[derive(Debug)]
pub struct Project {
  pub project_name: String,
  pub project_type: ProjectType,
  pub project_version: (u16, u16, u16),
  pub project_path: PathBuf,
}

#[derive(Debug)]
pub enum ProjectType {
  Binary,
  Library(LibraryType),
}

#[derive(Debug)]
pub enum LibraryType {
  Shared,
  Static,
}

pub trait Command {
  fn execute(&self);
}

impl Command for Project {
  fn execute(&self) {
    println!("{:?}", self);
  }
}

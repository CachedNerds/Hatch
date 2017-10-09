pub mod metadata;

use super::ErrorKind;
use super::fs;
use super::errors;

#[derive(Debug)]
pub enum RunKind {
  Create,
  Update,
}

#[derive(Debug)]
pub struct Project {
  pub name: String,
  pub build_type: ProjectType,
  pub metadata: metadata::Metadata,
  pub run_type: RunKind,
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

impl Project {
  pub fn manage_files(&self) -> Result<(), errors::Error> {
    match &self.run_type {
      &RunKind::Create => {
        match fs::create_dir(&self.metadata.path) {
          Ok(_) => { println!("New project generated"); Ok(()) },
          Err(e) => match e.kind() {
            ErrorKind::AlreadyExists =>
              Err(errors::Error::from("Project exists")),
            ErrorKind::NotFound =>
              Err(errors::Error::from("Invalid toolbox path")),
            _ => 
              Err(errors::Error::from("An error occured")),
          },
        }
      },
      &RunKind::Update => {
        Ok(())
      },
    }
  }
}

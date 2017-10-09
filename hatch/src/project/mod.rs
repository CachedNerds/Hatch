pub mod metadata;

use super::ErrorKind;
use super::fs;
use super::errors;

#[derive(Debug)]
pub struct Project {
  pub name: String,
  pub build_type: ProjectType,
  pub metadata: metadata::Metadata,
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
  pub fn create_project_files(&self) -> Result<(), errors::Error> {
    match fs::create_dir(&self.metadata.path) {
      Ok(a) => {
        println!("New project generated");
        Ok(())
      }
      Err(e) => match e.kind() {
        ErrorKind::AlreadyExists => {
          println!("{:?}", fs::read_dir(&self.metadata.path));
          Ok(())
        },
        ErrorKind::NotFound =>
          Err(errors::Error::from("Invalid toolbox path")),
        _ =>
          Err(errors::Error::from("An error occured")),
      }
    }
  }
}

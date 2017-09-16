#[derive(Debug)]
pub struct Project {
  pub project_name: String,
  pub project_type: ProjectType,
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

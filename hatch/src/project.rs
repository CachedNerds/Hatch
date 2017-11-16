use dtl::project as project_imp;

pub struct Project (project_imp::Project);

impl Project { 
  pub fn new() -> Project {
    Project(project_imp::Project::new())
  }

  pub fn name(&self) -> &str {
    self.0.name()
  }

  pub fn kind(&self) -> &ProjectKind {
    self.0.kind()
  }
  
  pub fn path(&self) -> &str {
    self.0.path()
  }
}

pub enum LibraryKind { Shared, Static }

pub enum ProjectKind { Binary, Library(LibraryKind) }

impl AsRef<ProjectKind> for ProjectKind {
  fn as_ref(&self) -> &ProjectKind {
    &self
  }
}

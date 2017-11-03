use dtl::project as project_imp;

use asset::Asset;

pub struct Project(project_imp::Project, Asset);

impl Project { 
  pub fn new(name: String, kind: ProjectKind, path: String) -> Project {
    Project(project_imp::Project::new(name, kind, path), Asset::new())
  }
}

pub enum LibraryKind { Shared, Static }

pub enum ProjectKind { Binary, Library(LibraryKind) }

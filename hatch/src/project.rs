use dtl::project as project_imp;

pub struct Project(project_imp::Project);

impl Project { 
}

pub enum LibraryKind { Shared, Static }

pub enum ProjectKind { Binary, Library(LibraryKind) }

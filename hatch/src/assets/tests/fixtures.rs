use project::{ Project, Arch, Target, BuildConfig, ProjectKind, LibraryKind };
use std::path::PathBuf;

pub fn library_project() -> Project {
  let config = BuildConfig::new(ProjectKind::Library(LibraryKind::Static),
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                vec![String::from("-v")],
                                Arch::X64,
                                Target::Release);

  let project = Project::new(String::from("test"),
                             String::from("0.1.0"),
                             config,
                             Vec::new(),
                             PathBuf::from("./"));

  project
}

pub fn binary_project() -> Project {
  let config = BuildConfig::new(ProjectKind::Binary,
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                vec![String::from("-v")],
                                Arch::X64,
                                Target::Release);

  let project = Project::new(String::from("test"),
                             String::from("0.1.0"),
                             config,
                             Vec::new(),
                             PathBuf::from("./"));

  project
}
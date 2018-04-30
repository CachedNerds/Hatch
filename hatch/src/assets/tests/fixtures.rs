use project::{Project, ProjectKind};
use project::build::{Config, Target};
use platform::arch::Arch;
use std::path::PathBuf;

pub fn project(kind: ProjectKind) -> Project {
    let config = Config::new(
        kind,
        String::from("g++"),
        vec![String::from("-c"), String::from("--std=c++1z")],
        vec![String::from("-v")],
        Arch::X64,
        Target::Release,
    );

    let project = Project::new(
        String::from("test"),
        String::from("0.1.0"),
        config,
        Vec::new(),
        PathBuf::from("./"),
    );

    project
}

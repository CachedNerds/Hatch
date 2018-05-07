use project::{Project, ProjectKind};
use platform::arch::Arch;
use std::path::PathBuf;
use project::CompilerOptions;
use project::Target;

pub fn project(kind: ProjectKind) -> Project {
    let config = CompilerOptions::new(
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

pub fn project_with_compiler_options(kind: ProjectKind, compiler_options: CompilerOptions) -> Project {
    Project::new(String::from("test"), String::from("0.1.0"), kind, compiler_options, PathBuf::from("./"))
}
use platform::arch::Arch;
use project::CompilerOptions;
use project::Target;
use project::{Project, ProjectKind};

pub fn project(kind: ProjectKind) -> Project {
    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        vec![String::from("-c"), String::from("--std=c++1z")].join(' '.to_string().as_str()),
        vec![String::from("-v")].join(' '.to_string().as_str()),
        Arch::X64,
        Target::Release,
    );

    let project = Project::new(
        String::from("test"),
        String::from("0.1.0"),
        kind,
        Some(compiler_options),
        None,
    );

    project
}

pub fn project_with_compiler_options(
    kind: ProjectKind,
    compiler_options: CompilerOptions,
) -> Project {
    Project::new(
        String::from("test"),
        String::from("0.1.0"),
        kind,
        Some(compiler_options),
        None,
    )
}

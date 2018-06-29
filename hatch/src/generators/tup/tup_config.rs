use project::ProjectKind;

fn project(name: &str) -> String {
    format!("PROJECT = {}", name)
}

fn lib_type(project_kind: &ProjectKind) -> String {
    let kind = match *project_kind {
        ProjectKind::Binary => "binary",
        ProjectKind::Static => "static",
        ProjectKind::Shared => "shared",
        ProjectKind::HeaderOnly => "header-only",
    };
    format!("LIB_TYPE = {}", kind)
}

pub fn file_name() -> String {
    String::from("config.tup")
}

pub fn make_string(name: &str, project_kind: &ProjectKind) -> String {
    [project(name), lib_type(project_kind)].join("\n")
}

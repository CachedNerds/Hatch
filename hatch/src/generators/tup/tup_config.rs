use project::ProjectKind;

fn name() -> String {
    String::from("config.tup")
}

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

fn make_config_string(project_kind: &ProjectKind) -> String {
    let kind = match *project_kind {
        ProjectKind::Binary => "binary",
        ProjectKind::Static => "static",
        ProjectKind::Shared => "shared",
        ProjectKind::HeaderOnly => "header-only",
    };

    format!("LIB_TYPE = {}", kind)
}

pub fn make_tup_config_string(name: &str, project_kind: &ProjectKind) -> String {
    [project(name), lib_type(project_kind)].join("\n")
}

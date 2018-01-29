use std::fs;
use std::io::Read;
use std::path::{ Path, PathBuf };

use yaml_rust::{ Yaml, YamlLoader };
use project::{ Project, LibraryKind, ProjectKind, Dependency };

use hatch_error::{
  HatchResult,
  ResultExt,
  MissingNameError,
  MissingBuildError,
  MissingVersionError,
  EmptyConfigError,
};

pub fn parse(path: &Path, name: String) -> HatchResult<Project> {
  let yml_vec = from_file(path.join(name).as_path())?;

  if yml_vec.is_empty() {
    return Err(EmptyConfigError)?;
  }

  let name: String;
  let kind: ProjectKind;
  let version: String;
  let deps: Vec<Dependency>;

  if let Some(n) = yml_vec[0]["name"].as_str() {
    name = n.to_owned();
  } else {
    return Err(MissingNameError)?;
  }

  if let Some(b) = yml_vec[0]["build"].as_str() {
    kind = match b {
      "static-lib" => ProjectKind::Library(LibraryKind::Shared),
      "shared-lib" => ProjectKind::Library(LibraryKind::Static),
      _ => ProjectKind::Binary
    }
  } else {
    return Err(MissingBuildError)?;
  }

  if let Some(v) = yml_vec[0]["version"].as_str() {
    version = v.to_owned();
  } else {
    return Err(MissingVersionError)?;
  }

  if let Some(d) = yml_vec[0]["deps"].as_hash() {
    deps = d
      .iter()
      .filter_map(|(_k, v)| v.as_str())
      .map(|v| Dependency::new(v.to_owned()))
      .collect();
  } else {
    deps = Vec::new();
  }

  Ok(Project::new(name, kind, version, deps, PathBuf::from(path)))
}

fn from_file(file_name: &Path) -> HatchResult<Vec<Yaml>> {
  let mut file = fs::File::open(&file_name).with_context(|_| {
    format!("failed to open file: `{}`", &file_name.display())
  })?;

  let mut contents = String::new();
  file.read_to_string(&mut contents).with_context(|_| {
    format!("failed to read contents of: `{}`", file_name.display())
  })?;

  let res = YamlLoader::load_from_str(&contents).compat().with_context(|e| {
    format!("Parsing error: `{}`", e)
  })?;

  Ok(res)
}

#[cfg(test)]
mod tests {
}

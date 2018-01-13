use std::fs;
use std::io::Read;
use std::ffi::OsStr;
use std::path::PathBuf;

use yaml_rust::{ Yaml, YamlLoader };
use project::{ Project, LibraryKind, ProjectKind };

use hatch_error::{
  HatchResult,
  ResultExt,
  HatchError,
  MissingNameError,
  MissingBuildError,
  MissingVersionError,
  EmptyConfigError
};

pub fn parse_all(path: &String) -> Vec<HatchResult<Project>> {
  match read_path(path) {
    Ok(files) => parse_many(path, get_project_names(extract_dirs(files))),
    Err(e) => vec![Err(e)],
  }
}

pub fn parse_one(path: &String) -> HatchResult<Project> {
  match from_file(path.to_owned() + "Hatch.yml") {
    Err(e) => Err(e),
    Ok(yml_vec) => parse(yml_vec),
  }
}

pub fn parse_many(path: &String, items: Vec<String>) -> Vec<HatchResult<Project>> {
  let yaml_result = items.into_iter().map(|p| {
    from_file(path.to_owned() + &p[..] + "/Hatch.yml")
  }).collect::<Vec<_>>();

  yaml_result.into_iter().map(|i| {
    match i {
      Err(e) => Err(e),
      Ok(yml_vec) => parse(yml_vec),
    }
  }).collect::<Vec<_>>()
}

fn from_file(file_name: String) -> HatchResult<Vec<Yaml>> {
  let mut file = fs::File::open(&file_name).with_context(|_| {
    format!("failed to open file: `{}`", &file_name)
  })?;

  let mut contents = String::new();
  file.read_to_string(&mut contents).with_context(|_| {
    format!("failed to read contents of: `{}`", file_name)
  })?;

  let res = YamlLoader::load_from_str(&contents).compat().with_context(|e| {
    format!("Parsing error: `{}`", e)
  })?;

  Ok(res)
}

fn get_project_names(dir_paths: Vec<PathBuf>) -> Vec<String> {
  dir_paths.iter()
    .filter_map(|i| i.file_name())
    .map(OsStr::new)
    .filter_map(|i| i.to_str())
    .map(String::from)
    .collect()
}

fn extract_dirs(iter: fs::ReadDir) -> Vec<PathBuf> {
  iter.filter_map(|i| i.ok())
    .into_iter()
    .map(|i| i.path())
    .filter(|i| i.is_dir())
    .collect()
}

fn read_path(path: &str) -> HatchResult<fs::ReadDir> {
  let res = fs::read_dir(path).with_context(|_| {
    format!("failed to read directory `{}`", path)
  })?;

  Ok(res)
}

fn parse(yml_vec: Vec<Yaml>) -> HatchResult<Project> {
  if yml_vec.len() == 0 {
    return Err(EmptyConfigError)?;
  }

  let name: String;
  let kind: ProjectKind;
  let version: String;

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

  Ok(Project::new(name, kind, version))
}



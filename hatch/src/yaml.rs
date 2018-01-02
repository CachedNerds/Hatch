use std::fs;
use std::io::Read;

use HatchResult;
use yaml_rust::{ Yaml, YamlLoader };
use project::{ Project, LibraryKind, ProjectKind };

use hatch_error::{
  HatchError,
  MissingNameError,
  MissingBuildError,
  MissingVersionError,
  EmptyConfigError
};

use self::HatchError::{ Io, Parsing };

pub fn parse_one(path: String) -> HatchResult<Project> {
  match from_file(path.to_owned() + "Hatch.yml") {
    Err(e) => Err(e),
    Ok(yml_vec) => parse(yml_vec),
  }
}

pub fn parse_many(path: String, items: Vec<String>) -> Vec<HatchResult<Project>> {
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

fn from_file(file_name: String) -> Result<Vec<Yaml>, HatchError> {
  let parsed = fs::File::open(file_name).and_then(|mut file| {
    let mut contents = String::new();
    file.read_to_string(&mut contents).map(|_| YamlLoader::load_from_str(&contents))
  });

  match parsed {
    Err(io_error) => Err(Io(io_error)),
    Ok(yaml_result) => match yaml_result {
      Err(scanner_error) => Err(Parsing(scanner_error)),
      Ok(vec_yaml) => Ok(vec_yaml)
    }
  }
}

fn parse(yml_vec: Vec<Yaml>) -> HatchResult<Project> {
  if yml_vec.len() == 0 {
    return Err(HatchError::EmptyConfig(EmptyConfigError));
  }

  let name: String;
  let kind: ProjectKind;
  let version: String;

  if let Some(n) = yml_vec[0]["name"].as_str() {
    name = n.to_owned();
  } else {
    return Err(HatchError::MissingName(MissingNameError));
  }

  if let Some(b) = yml_vec[0]["build"].as_str() {
    kind = match b {
      "static-lib" => ProjectKind::Library(LibraryKind::Shared),
      "shared-lib" => ProjectKind::Library(LibraryKind::Static),
      _ => ProjectKind::Binary
    }
  } else {
    return Err(HatchError::MissingBuild(MissingBuildError));
  }

  if let Some(v) = yml_vec[0]["version"].as_str() {
    version = v.to_owned();
  } else {
    return Err(HatchError::MissingVersion(MissingVersionError));
  }

  Ok(Project::new(name, kind, version))
}

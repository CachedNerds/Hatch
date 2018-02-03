use std::fs;
use std::io::Read;
use std::path::{ Path, PathBuf };
use yaml_rust::{ Yaml, YamlLoader };
use project::{ Project, LibraryKind, ProjectKind };
use project::build::{ Target, Config };
use deps::dependency::Dependency;
use platform::arch::Arch;

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
  let version: String;
  let mut kind: ProjectKind = ProjectKind::Library(LibraryKind::Static);
  let mut compiler: String = String::from("g++");
  let mut compiler_flags: Vec<String> = vec![String::from("-c")];
  let mut linker_flags: Vec<String> = vec![String::from("-v")];
  let mut arch: Arch = Arch::X64;
  if let Some(architecture) = Arch::architecture() {
    arch = architecture;
  }
  let mut target: Target = Target::Debug;
  let deps: Vec<Dependency>;

  if let Some(n) = yml_vec[0]["name"].as_str() {
    name = n.to_owned();
  } else {
    return Err(MissingNameError)?;
  }

  if let Some(v) = yml_vec[0]["version"].as_str() {
    version = v.to_owned();
  } else {
    return Err(MissingVersionError)?;
  }

  if let Some(configurations) = yml_vec[0]["build"].as_hash() {
    let items = configurations
      .iter()
      .map(|(k, v)| {
        (k.as_str(), v.as_str())
      })
      .filter(|&(k, v)| {
        k.is_some() && v.is_some()
      })
      .map(|(k, v)| {
        (k.unwrap(), v.unwrap())
      }).collect::<Vec<(&str, &str)>>();

    for (key, value) in items {
      match key {
        "kind" => {
          kind = match value {
            "static-lib" => ProjectKind::Library(LibraryKind::Static),
            "shared-lib" => ProjectKind::Library(LibraryKind::Shared),
            _ => ProjectKind::Binary
          }
        },
        "compiler" => {
          compiler = String::from(value);
        },
        "compiler_flags" => {
          compiler_flags =
            String::from(value)
              .split_whitespace()
              .map(String::from)
              .collect();
        },
        "linker_flags" => {
          linker_flags =
            String::from(value)
              .split_whitespace()
              .map(String::from)
              .collect();
        },
        "arch" => {
          match value {
            "x64" => {
              arch =Arch::X64
            },
            "x86" => {
              arch = Arch::X86
            },
            _ => {
              // use default
            }
          }
        },
        "target" => {
          match value {
            "debug" => {
              target = Target::Debug
            },
            "release" => {
              target = Target::Release
            },
            _ => {
              // use default
            }
          }
        },
        _ => {}
      }
    }
  } else {
    return Err(MissingBuildError)?;
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

  let config = Config::new(kind, compiler, compiler_flags, linker_flags, arch, target);
  Ok(Project::new(name, version, config, deps, PathBuf::from(path)))
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

#[cfg(test)]
mod tests;

pub mod dependency;

use hatch_error::{ HatchResult, HatchError };
use git2::Repository;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use task;
use self::dependency::Dependency;
use locations::hatchfile_path;
use std::fs::File;
use std::io::Read;
use project::Project;
use serde_yaml;
use hatch_error::SerdeYamlError;

pub fn clone_dep(url: &str, path: &Path) {
  let _ = Repository::clone(url, path);
}

fn walk(path: &Path,
        callback: &mut FnMut(&Path) -> HatchResult<bool>) -> HatchResult<()>
{
  if !callback(path)? {
    return Ok(())
  }

  let dirs = fs::read_dir(path)?;

  for dir in dirs {
    let dir = dir?;
    if dir.file_type()?.is_dir() {
      walk(&dir.path(), callback)?;
    }
  }
  Ok(())
}

pub fn clone_project_deps(path: &Path,
                          dependencies: &Vec<Dependency>) -> HatchResult<()>
{
  let mut visited: HashSet<String> = HashSet::new();
  let mut errored: Vec<HatchError> = Vec::new();

  // All dependencies are cloned into here
  let registry = &path;

  // Clone the dependencies specified on the command line
  dependencies.iter().for_each(|dep| {
    clone_dep(&dep.url(), &path.join(&dep.name()));
  });

  // Clone the dependencies dependencies
  walk(path, &mut |dir| {
    // ignore hidden directories and ..
    let name = dir.file_name().and_then(|s| s.to_str());
    if name.map(|s| s.starts_with('.')) == Some(true) {
      return Ok(false)
    }

    // Grab the path to the Hatch.yml starting at `dir`
    let hatchfile = hatchfile_path(dir);

    if hatchfile.exists() {
      clone_nested_project_deps(&registry, &dir, &mut errored, &mut visited);
    }
    Ok(true)
  })?;

  if !errored.is_empty() {
    errored.into_iter().for_each(|e| {
      println!("ERROR: {}", e)
    });
  }

  Ok(())
}

fn clone_nested_project_deps(registry: &Path,
                             path: &Path,
                             errored: &mut Vec<HatchError>,
                             visited: &mut HashSet<String>)
{

  // TODO: Do one of these:
  // 1. Put this back in task::read_project
  // 2. Make a constructor for Project that takes a &Path
  let mut data = String::new();
  let mut file = File::open(&path);
  file.unwrap().read_to_string(&mut data);
  match serde_yaml::from_str::<Project>(&data) {
    Err(e) => {
      // TODO: For some reason, I can't push this error
      // errored.push(SerdeYamlError{});
    },
    Ok(current_project) => {
      if !visited.contains( &current_project.name().to_owned()) {
        current_project.dependencies().iter().for_each(|dep| {
          clone_dep(&dep.url(), &registry.join(dep.name()).as_path());
        });
        let _ = visited.insert(current_project.name().to_owned());
      }
    }
  }

//  match task::read_project(path) {
//    Err(e) => {
//      errored.push(e);
//    },
//    Ok(current_project) => {
//      if !visited.contains(&current_project.name().to_owned()) {
//        current_project.dependencies().iter().for_each(|dep| {
//          clone_dep(&dep.url(), &registry.join(dep.name()).as_path());
//        });
//        let _ = visited.insert(current_project.name().to_owned());
//      }
//    }
//  }
}

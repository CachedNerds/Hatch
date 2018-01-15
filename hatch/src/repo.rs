use hatch_error::{ HatchResult, ResultExt };
use git2::Repository;
use std::collections::HashSet;
use project::{ Project, Dependency };
use std::fs;
use std::path::{ Path, PathBuf, Component };
use yaml;

pub fn modules_path(base: &Path) -> PathBuf {
  base.join("hatch_modules")
}

pub fn hatchfile_path(base: &Path) -> PathBuf {
  base.join("Hatch.yml")
}

pub fn clone_dep(repo: &(String, String), path: &Path) {
  Repository::clone(&repo.0, path.join(&repo.1));
}

fn walk(path: &Path, callback: &mut FnMut(&Path) -> HatchResult<bool>) -> HatchResult<()> {
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
                          user_defined_deps: &Vec<Dependency>) -> HatchResult<Vec<Project>> 
{
  let mut visited: HashSet<String> = HashSet::new();
  // Clone the dependencies specified on the command line
  user_defined_deps.iter().for_each(|dep| {
    clone_dep(&dep.as_pair(), path);
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
      return clone_nested_project_deps(&dir, &mut visited);
    }
    Ok(true)
  })?;

  Ok(Vec::new())
}

fn clone_nested_project_deps(path: &Path, visited: &mut HashSet<String>) -> HatchResult<bool> {
  let current_project = yaml::parse_one(hatchfile_path(path).as_path())?;
  if !visited.insert(current_project.name().to_owned()) {
    current_project.deps().iter().for_each(|dep| {
      clone_dep(&dep.as_pair(), &modules_path(path).as_path());
    });
    Ok(true)
  } else {
    Ok(false)
  }
}

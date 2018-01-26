use hatch_error::HatchResult;
use git2::Repository;
use std::collections::HashSet;
use project::Dependency;
use std::fs;
use std::path::{ Path, PathBuf };
use task;

pub fn modules_path(base: &Path) -> PathBuf {
  base.join("hatch_modules")
}

pub fn hatchfile_path(base: &Path) -> PathBuf {
  base.join("Hatch.yml")
}

pub fn clone_dep(url: &str, path: &Path) {
  Repository::clone(url, path);
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
                          user_defined_deps: &Vec<Dependency>) -> HatchResult<()> 
{
  let mut visited: HashSet<String> = HashSet::new();

  // Clone the dependencies specified on the command line
  user_defined_deps.iter().for_each(|dep| {
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
      return clone_nested_project_deps(&dir, &mut visited);
    }
    Ok(true)
  })?;

  Ok(())
}

fn clone_nested_project_deps(path: &Path, visited: &mut HashSet<String>) -> HatchResult<bool> {
  let current_project = task::read_project(path)?;
  current_project.deps().iter().for_each(|dep| {
    // If we have already pulled in a dep, dont do it again
    if !visited.insert(current_project.name().to_owned()) {
      clone_dep(&dep.url(), &modules_path(path).join(dep.name()).as_path());
    }
  });
  Ok(true)
}

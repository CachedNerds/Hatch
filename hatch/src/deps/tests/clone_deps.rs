use deps;
use deps::dependency::Dependency;
use std::path::{ PathBuf };
use std::fs;

#[test]
fn clone_one_dep() {
  let registry = PathBuf::from("TestProject/hatch_modules");

  let org = String::from("https://github.com/CachedNerds/");
  let dep = vec![Dependency::new(String::from(org.to_owned() + "Meta.git"))];

  deps::clone_project_deps(registry.as_path(), &dep);

  let actual_files_and_dirs = fs::read_dir(registry).unwrap().map(|e| {
    e.unwrap().path().as_path().to_string_lossy().into()
  }).collect::<Vec<String>>();

  let expected_files_and_dirs = vec!["foo".to_owned()];

  assert_eq!(actual_files_and_dirs, expected_files_and_dirs);
}

#[test]
fn clone_deps() {
  let registry = PathBuf::from("TestProject/hatch_modules");
  let org = String::from("https://github.com/CachedNerds/");
  
  let deps = vec![
    Dependency::new(String::from(org.to_owned() + "Meta.git")),
    Dependency::new(String::from(org.to_owned() + "Conversion.git"))
  ];

  deps::clone_project_deps(registry.as_path(), &deps);

  let actual_files_and_dirs = fs::read_dir(registry).unwrap().map(|e| {
    e.unwrap().path().as_path().to_string_lossy().into()
  }).collect::<Vec<String>>();

  let expected_files_and_dirs = vec!["foo".to_owned()];

  assert_eq!(actual_files_and_dirs, expected_files_and_dirs);
}

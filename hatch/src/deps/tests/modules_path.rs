use deps;
use std::path::{ Path, PathBuf };

#[test]
fn modules_path() {
  let path = Path::new("TestProject");

  let expected_path = PathBuf::from("TestProject").join("hatch_modules");
  let actual_path = deps::modules_path(path);

  assert_eq!(expected_path, actual_path);
}

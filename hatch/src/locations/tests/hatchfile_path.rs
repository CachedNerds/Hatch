use deps;
use std::path::{ Path, PathBuf };

#[test]
fn hatchfile_path() {
  let path = Path::new("TestProject");
  
  let expected_path = PathBuf::from("TestProject").join("Hatch.yml");
  let actual_path = deps::hatchfile_path(path);
  
  assert_eq!(expected_path, actual_path);
}

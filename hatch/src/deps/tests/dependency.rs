use deps::dependency::Dependency;

#[test]
fn dependency_name() {
  let dep =
    Dependency::new("https://github.com/CachedNerds/TestProject.git".to_owned());

  let expected_name = "TestProject";
  let actual_name = dep.name();

  assert_eq!(expected_name, actual_name); 
}

#[test]
fn dependency_url() {
  let dep =
    Dependency::new("https://github.com/CachedNerds/TestProject.git".to_owned());

  let expected_url = "https://github.com/CachedNerds/TestProject.git";
  let actual_url = "https://github.com/CachedNerds/TestProject.git";

  assert_eq!(expected_url, actual_url); 
}

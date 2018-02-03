use project::Dependency;

#[test]
fn dependency_name() {
  let url = "https://github.com/CachedNerds/TestProject.git"; 
  let dep = Dependency::new(url.to_owned());

  let expected_name = "TestProject";
  let actual_name = dep.name();

  assert_eq!(expected_name, actual_name); 
}

#[test]
fn dependency_url() {
  let url = "https://github.com/CachedNerds/TestProject.git"; 
  let dep = Dependency::new(url.to_owned());

  let expected_url = "https://github.com/CachedNerds/TestProject.git";
  let actual_url = dep.url();

  assert_eq!(expected_url, actual_url); 
}

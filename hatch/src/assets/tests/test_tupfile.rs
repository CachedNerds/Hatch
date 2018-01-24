use assets::test_tupfile::Tupfile;

#[test]
fn build_test_tupfile() {
  assert_eq!(".gitignore", Tupfile::new().to_string());
}

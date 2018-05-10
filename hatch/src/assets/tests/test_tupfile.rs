use project::ProjectKind;
use generators::tup::test_tupfile::make_test_tupfile_string;

#[test]
fn build_test_tupfile() {
    assert_eq!(".gitignore", make_test_tupfile_string());
}

use generators::tup::test_tupfile;

#[test]
fn test_tupfile_file_name() {
    assert_eq!("Tupfile", test_tupfile::file_name())
}

#[test]
fn generate_test_tupfile() {
    assert_eq!(".gitignore", test_tupfile::make_string());
}

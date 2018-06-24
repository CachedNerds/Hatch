use generators::tup::catch_definition;

#[test]
fn catch_definition_file_name() {
    assert_eq!("catch.cpp", catch_definition::file_name());
}

#[test]
fn generate_catch_definition() {
    assert_eq!(
        "#define CATCH_CONFIG_MAIN\n#include \"catch.hpp\"",
        catch_definition::make_string()
    );
}

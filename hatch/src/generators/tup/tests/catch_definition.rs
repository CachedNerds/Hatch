use generators::tup::catch_definition::make_catch_definition_string;

#[test]
fn build_catch_definition() {
    assert_eq!(
        "#define CATCH_CONFIG_MAIN\n#include \"catch.hpp\"",
        make_catch_definition_string()
    );
}

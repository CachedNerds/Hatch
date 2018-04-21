use assets::catch_definition::CatchDefinition;

#[test]
fn build_catch_definition() {
    assert_eq!(
        "#define CATCH_CONFIG_MAIN\n#include \"catch.hpp\"",
        CatchDefinition::new().to_string()
    );
}

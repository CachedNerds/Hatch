pub struct CatchDefinition;

impl CatchDefinition {
    pub fn new() -> CatchDefinition {
        CatchDefinition
    }

    pub fn name() -> String {
        String::from("catch.cpp")
    }
}

impl ToString for CatchDefinition {
    fn to_string(&self) -> String {
        String::from("#define CATCH_CONFIG_MAIN\n#include \"catch.hpp\"")
    }
}

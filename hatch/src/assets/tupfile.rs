pub struct Tupfile;

impl Tupfile {
  pub fn new() -> Tupfile {
    Tupfile
  }

  pub fn name() -> String {
    String::from("Tupfile")
  }
}

impl ToString for Tupfile {
  fn to_string(&self) -> String {
    String::from(
"include config.tup
include_rules

# override build variables
# VARIABLE = new_value

# define custom build variables

# Compile Source
: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

# Archive Source
: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>

# Compile Tests
: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

# Create Link Executable
: $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test")
  }
}
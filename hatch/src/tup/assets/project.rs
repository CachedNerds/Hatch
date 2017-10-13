use tup::{ ProjectKind };
use tup::assets::{ Assets };
use ProjectKind::{ Library, Binary };
use LibraryKind::{ SharedLib, StaticLib };

pub struct ProjectAssets {
  file_path: String,
  file_contents: String,
}

impl Assets for ProjectAssets {
  fn path(&self) -> &str {
    &self.file_path.as_str()
  }

  fn contents(&self) -> &str {
    &self.file_contents.as_str()
  }
}

impl ProjectAssets {
  pub fn config(path: &str, name: &str, project_type: &ProjectKind) -> ProjectAssets {
    let file_path = path.to_string() + "/" + name + "/config.tup";

    let file_contents = match project_type {
      &Library(SharedLib) => format!( // {{{
"PROJECT = {}
LIB_TYPE = shared", // }}}
      &name).to_string(),
      
      &Library(StaticLib) => format!( // {{{
"PROJECT = {}
LIB_TYPE = static", // }}}
        &name).to_string(),
      
      &Binary => format!( // {{{
"PROJECT = {}", // }}}
        &name).to_string(),
    };

    ProjectAssets { file_path, file_contents }
  }

  pub fn tupfile(path: &str, name: &str) -> ProjectAssets {
    let file_path = path.to_string() + "/" + name + "/Tupfile";
    let file_contents = // {{{
"# order matters
include config.tup
include_rules

# override build variables
# VARIABLE = new_value

# define custom build variables

# Compile Source
: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_OUT)/%B.o

# Archive Source
: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_OUT)/$(PROJECT_LIB) ../<$(PROJECT)>

# Compile Tests
: foreach $(TEST_FILES) |> !compile |> $(TEST_OUT)/%B.o

# Create Link Executable
: $(TEST_OBJ_FILES) $(SOURCE_OUT)/$(PROJECT_LIB) |> !link |> $(TEST_OUT)/$(PROJECT).test".to_string(); // }}}
    
    ProjectAssets { file_path, file_contents }
  }
}

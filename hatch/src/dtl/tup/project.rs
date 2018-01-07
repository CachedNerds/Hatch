use dtl::tup::{ Assets };
use project::{ LibraryKind, ProjectKind, Project };

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
  pub fn config(project: &Project) -> ProjectAssets {
    let file_path = "C++/libs/".to_owned() + project.name() + "/config.tup";

    let file_contents = match *project.kind() {
      ProjectKind::Library(ref lib_type) => match lib_type {
        &LibraryKind::Shared => format!( // {{{
"PROJEct = {}
LIB_TYpe = shared", // }}}
        project.name()).to_string(),

        &LibraryKind::Static => format!( // {{{
"PROJECT = {}
LIB_TYPE = static", // }}}
        project.name()).to_string(),
      },

      ProjectKind::Binary => format!( // {{{
"PROJECT = {}", // }}}
    project.name()).to_string(),
    };

    ProjectAssets { file_path, file_contents }
  }

  pub fn tupfile(project: &Project) -> ProjectAssets {
    let file_path = "C++/libs/".to_owned() + project.name() + "/Tupfile";
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

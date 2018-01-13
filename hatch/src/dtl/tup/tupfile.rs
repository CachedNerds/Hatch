//     pub fn tupfile(project: &Project) -> ProjectAsset {
//     let file_path = "C++/libs/".to_owned() + project.name() + "/Tupfile";
//     let file_contents = // {{{
// "# order matters
// include config.tup
// include_rules
// # override build variables
// # VARIABLE = new_value
// # define custom build variables
// # Compile Source
// : foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_OUT)/%B.o
// # Archive Source
// : $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_OUT)/$(PROJECT_LIB) ../<$(PROJECT)>
// # Compile Tests
// : foreach $(TEST_FILES) |> !compile |> $(TEST_OUT)/%B.o
// # Create Link Executable
// : $(TEST_OBJ_FILES) $(SOURCE_OUT)/$(PROJECT_LIB) |> !link |> $(TEST_OUT)/$(PROJECT).test".to_string(); // }}}

//     ProjectAsset { file_path, file_contents }
//   }

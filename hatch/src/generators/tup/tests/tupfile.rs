use generators::tup::tests::fixtures;
use project::ProjectKind;
use generators::tup::tupfile::make_tupfile_string;

#[test]
fn build_library_tupfile() {
    let project = fixtures::project(ProjectKind::Static);

    let contents = String::from(
        "include config.tup
include_rules

: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>

: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

: $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
    );

    assert_eq!(contents, make_tupfile_string(project.kind()));
}

#[test]
fn build_binary_tupfile() {
    let project = fixtures::project(ProjectKind::Binary);

    let contents = String::from(
        "include config.tup
include_rules

: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

: $(SOURCE_OBJ_FILES) |> !link |> $(SOURCE_TARGET)/$(PROJECT)

: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

: $(TEST_OBJ_FILES) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
    );

    assert_eq!(contents, make_tupfile_string(project.kind()).to_string());
}

use generators::tup::tests::fixtures;
use generators::tup::tupfile;
use project::ProjectKind;

#[test]
fn tupfile_file_name() {
    assert_eq!("Tupfile", tupfile::file_name())
}

#[test]
fn generate_library_tupfile() {
    let project = fixtures::project(ProjectKind::Static);

    let contents = String::from(
        "include config.tup
include_rules

: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>

: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

: $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
    );

    assert_eq!(contents, tupfile::make_string(project.kind()));
}

#[test]
fn generate_binary_tupfile() {
    let project = fixtures::project(ProjectKind::Binary);

    let contents = String::from(
        "include config.tup
include_rules

: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

: $(SOURCE_OBJ_FILES) |> !link |> $(SOURCE_TARGET)/$(PROJECT)

: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

: $(TEST_OBJ_FILES) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
    );

    assert_eq!(contents, tupfile::make_string(project.kind()).to_string());
}

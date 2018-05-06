use assets::ProjectAsset;
use project::ProjectKind;

pub fn make_tupfile_string(kind: &ProjectKind) -> String {
    let mut tokens = Vec::new();

    let includes = String::from("include config.tup\ninclude_rules\n");
    let compile_source =
        String::from(": foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o\n");
    let compile_tests =
        String::from(": foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o\n");

    tokens.push(includes);
    tokens.push(compile_source);

    match kind {
        ProjectKind::Binary => {
            tokens.push(String::from(
                ": $(SOURCE_OBJ_FILES) |> !link |> $(SOURCE_TARGET)/$(PROJECT)\n",
            ));
        }
        ProjectKind::Static | ProjectKind::Shared => {
            tokens.push(String::from(": $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>\n"));
        }
        _ => panic!("should never get here!"),
    }

    tokens.push(compile_tests);

    match kind {
        ProjectKind::Binary => tokens.push(String::from(
            ": $(TEST_OBJ_FILES) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
        )),
        ProjectKind::Static | ProjectKind::Shared => {
            tokens.push(String::from(": $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test"));
        }
        _ => panic!("should never get here!"),
    };

    tokens
        .iter()
        .map(|token| token.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

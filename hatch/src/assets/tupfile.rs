use project::{LibraryKind, ProjectKind};

pub struct Tupfile {
    kind: ProjectKind,
}

impl Tupfile {
    pub fn new(kind: &ProjectKind) -> Tupfile {
        let copy_kind = match *kind {
            ProjectKind::Binary => ProjectKind::Binary,
            ProjectKind::Library(LibraryKind::Static) => ProjectKind::Library(LibraryKind::Static),
            ProjectKind::Library(LibraryKind::Shared) => ProjectKind::Library(LibraryKind::Shared),
        };

        Tupfile { kind: copy_kind }
    }

    pub fn name() -> String {
        String::from("Tupfile")
    }
}

impl ToString for Tupfile {
    fn to_string(&self) -> String {
        let mut tokens = Vec::new();

        let includes = String::from("include config.tup\ninclude_rules\n");
        let compile_source =
            String::from(": foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o\n");
        let compile_tests =
            String::from(": foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o\n");

        tokens.push(includes);
        tokens.push(compile_source);

        match self.kind {
            ProjectKind::Binary => {
                tokens.push(String::from(
                    ": $(SOURCE_OBJ_FILES) |> !link |> $(SOURCE_TARGET)/$(PROJECT)\n",
                ));
            }
            ProjectKind::Library(_) => {
                tokens.push(String::from(": $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>\n"));
            }
        }

        tokens.push(compile_tests);

        match self.kind {
            ProjectKind::Binary => {
                tokens.push(String::from(
                    ": $(TEST_OBJ_FILES) |> !link |> $(TEST_TARGET)/$(PROJECT).test",
                ));
            }
            ProjectKind::Library(_) => {
                tokens.push(String::from(": $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test"));
            }
        }

        tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

use project::ProjectKind;

pub struct Tupfile {
  kind: ProjectKind
}

impl Tupfile {
  pub fn new(kind: &ProjectKind) -> Tupfile {
    Tupfile { kind: kind.clone() }
  }

  pub fn name() -> String {
    String::from("Tupfile")
  }
}

impl ToString for Tupfile {
  fn to_string(&self) -> String {
    let mut tokens = Vec::new();

    let includes = String::from("include config.tup\ninclude_rules\n");
    let compile_source = String::from(": foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o\n");
    let compile_tests = String::from(": foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o\n");

    tokens.push(includes);
    tokens.push(compile_source);

    match self.kind {
      ProjectKind::Binary => {
        tokens.push(String::from(": $(SOURCE_OBJ_FILES) |> !link |> $(SOURCE_TARGET)/$(PROJECT)\n"));
      },
      ProjectKind::Static | ProjectKind::Shared => {
        tokens.push(String::from(": $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>\n"));
      },
      _ => ()
    }

    tokens.push(compile_tests);

    match self.kind {
      ProjectKind::Binary => {
        tokens.push(String::from(": $(TEST_OBJ_FILES) |> !link |> $(TEST_TARGET)/$(PROJECT).test"));
      },
      ProjectKind::Static | ProjectKind::Shared => {
        tokens.push(String::from(": $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test"));
      },
      _ => ()
    }

    tokens.iter().map(|token| token.as_str()).collect::<Vec<_>>().join("\n")
  }
}

use assets::Arch;
use project::LibraryKind;

pub struct Tuprules {
  compiler: String,
  debug: bool,
  arch: Arch,
  compiler_version: String,
  lib_type: LibraryKind,
}

impl Tuprules {
  pub fn new(compiler: String, debug: bool, arch: Arch, compiler_version: String, lib_type: &LibraryKind) -> Tuprules {
    let copy_lib_type = match *lib_type {
      LibraryKind::Static => LibraryKind::Static,
      LibraryKind::Shared => LibraryKind::Shared
    };

    Tuprules { compiler, debug, arch, compiler_version, lib_type: copy_lib_type }
  }

  pub fn name() -> String {
    String::from("Tuprules.tup")
  }

  fn arch_flag(arch: &Arch) -> String {
    match *arch {
      Arch::X64 => String::from("-m64"),
      Arch::X32 => String::from("-m32"),
    }
  }

  fn type_flag(lib_type: &LibraryKind) -> String {
    match *lib_type {
      LibraryKind::Static => String::from("-static"),
      LibraryKind::Shared => String::from("-dynamic"),
    }
  }
}

impl ToString for Tuprules {
  fn to_string(&self) -> String {
    let mut tokens = Vec::new();
    tokens.push(String::from(".gitignore"));

    let compiler_token = String::from("CC = ") + self.compiler.as_str();
    tokens.push(compiler_token);

    if self.debug {
      let debug_token = String::from("CFLAGS += -g");
      tokens.push(debug_token);
    }

    let arch_flag = Tuprules::arch_flag(&self.arch);
    let arch_token = String::from("ARCH = ") + arch_flag.as_str();
    tokens.push(arch_token);

    tokens.push(String::from("CFLAGS += $(ARCH)"));

    let cflags_version = String::from("CFLAGS += -std=") + self.compiler_version.as_str();
    tokens.push(cflags_version);

    tokens.push(String::from("CFLAGS += -c"));
    tokens.push(String::from("CFLAGS += -I ../../"));

    tokens.push(String::from("LINKFLAGS += $(ARCH)"));

    let link_flags_type = String::from(
"ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += ".to_owned() + Tuprules::type_flag(&self.lib_type).as_str() + "
endif");
    tokens.push(link_flags_type);

    tokens.push(String::from(
"LINKFLAGS += -v
SOURCE = src
SOURCE_OUT = target
SOURCE_FILES = $(SOURCE)/*.cpp
SOURCE_OBJ_FILES = $(SOURCE_OUT)/*.o
TEST = test
TEST_OUT = $(TEST)/target
TEST_FILES = $(TEST)/$(SOURCE)/*.cpp
TEST_OBJ_FILES = $(TEST_OUT)/*.o
# macros
!compile = |> $(CC) $(CFLAGS) %f -o %o |>
!archive = |> ar crs %o %f |>
!link = |> $(CC) $(LINKFLAGS) %f -o %o |>
# includes the STATIC and SHARED variables for the target platform
include @(TUP_PLATFORM).tup
ifeq ($(LIB_TYPE),static)
  EXTENSION = $(STATIC)
else
  ifeq ($(LIB_TYPE),shared)
    EXTENSION = $(SHARED)
  else
    ifeq ($(LIB_TYPE),both)
      EXTENSION = both
    endif
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)"));

    tokens.iter().map(|token| token.as_str()).collect::<Vec<_>>().join("\n")
  }
}
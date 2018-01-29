use project::{ ProjectKind, LibraryKind, BuildConfig, Arch, Target };

pub struct Tuprules {
  kind: ProjectKind,
  compiler: String,
  compiler_flags: String,
  linker_flags: String,
  arch: Arch,
  target: Target
}

impl Tuprules {
  pub fn new(config: &BuildConfig) -> Tuprules {
    let copy_kind = match *config.kind() {
      ProjectKind::Binary => ProjectKind::Binary,
      ProjectKind::Library(LibraryKind::Static) => ProjectKind::Library(LibraryKind::Static),
      ProjectKind::Library(LibraryKind::Shared) => ProjectKind::Library(LibraryKind::Shared)
    };

    let copy_arch = match *config.arch() {
      Arch::X64 => Arch::X64,
      Arch::X32 => Arch::X32
    };

    let copy_target = match *config.target() {
      Target::Debug => Target::Debug,
      Target::Release => Target::Release
    };

    Tuprules {
      kind: copy_kind,
      compiler: String::from(config.compiler()),
      compiler_flags: config.compiler_flags().join(" "),
      linker_flags: config.linker_flags().join(" "),
      arch: copy_arch,
      target: copy_target
    }
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

    let compiler_token = format!("CC = {}", self.compiler);
    tokens.push(compiler_token);

    match self.target {
      Target::Debug => {
        let debug_token = String::from("CFLAGS += -g");
        tokens.push(debug_token);
      },
      _ => {}
    }

    let arch_flag = Tuprules::arch_flag(&self.arch);
    let arch_token = format!("ARCH = {}", arch_flag);
    tokens.push(arch_token);

    tokens.push(String::from("CFLAGS += $(ARCH)"));
    let compiler_flags = format!("CFLAGS += {}", self.compiler_flags);
    tokens.push(compiler_flags);

    tokens.push(String::from("LINKFLAGS += $(ARCH)"));
    let linker_flags = format!("LINKFLAGS += {}", self.linker_flags);
    tokens.push(linker_flags);

    match self.kind {
      ProjectKind::Library(ref lib_kind) => {
        let link_flags_type = format!(
"ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += {}
endif", Tuprules::type_flag(lib_kind));

        tokens.push(link_flags_type);
      },
      ProjectKind::Binary => {

      }
    }

    tokens.push(String::from(
"SOURCE = src
TARGET = target
SOURCE_TARGET = $(TARGET)
SOURCE_FILES = $(SOURCE)/*.cpp
SOURCE_OBJ_FILES = $(SOURCE_TARGET)/*.o
TEST = test
TEST_TARGET = $(TEST)/$(TARGET)
TEST_FILES = $(TEST)/$(SOURCE)/*.cpp
TEST_OBJ_FILES = $(TEST_TARGET)/*.o
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

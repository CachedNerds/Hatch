use project::ProjectKind;
use project::build::{ Config, Target };
use platform::arch::Arch;

pub struct Tuprules {
  kind: ProjectKind,
  compiler: String,
  compiler_flags: String,
  linker_flags: String,
  arch: Arch,
  target: Target
}

impl Tuprules {
  pub fn new(config: &Config) -> Tuprules {

    let copy_arch = match *config.arch() {
      Arch::X64 => Arch::X64,
      Arch::X86 => Arch::X86
    };

    let copy_target = match *config.target() {
      Target::Debug => Target::Debug,
      Target::Release => Target::Release
    };

    Tuprules {
      kind: config.kind().clone(),
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
      Arch::X86 => String::from("-m32"),
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

    if !self.compiler_flags.is_empty() {
      let compiler_flags = format!("CFLAGS += {}", self.compiler_flags);
      tokens.push(compiler_flags);
    }

    tokens.push(String::from("LINKFLAGS += $(ARCH)"));

    if !self.linker_flags.is_empty() {
      let linker_flags = format!("LINKFLAGS += {}", self.linker_flags);
      tokens.push(linker_flags);
    }

    match self.kind {
      ProjectKind::Static => {
        let link_flags_type = String::from(
"ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -static
endif");
        tokens.push(link_flags_type);
      }
      ProjectKind::Shared => {
        let link_flags_type = String::from(
"ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -dynamic
endif");
        tokens.push(link_flags_type);
      }
      _ => ()
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
include @(TUP_PLATFORM).tup"));

    match self.kind {
      ProjectKind::Static | ProjectKind::Shared => {
        tokens.push(String::from(
"ifeq ($(LIB_TYPE),static)
  EXTENSION = $(STATIC)
else
  ifeq ($(LIB_TYPE),shared)
    EXTENSION = $(SHARED)
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)"));
      },
      _ => {}
    }

    tokens.iter().map(|token| token.as_str()).collect::<Vec<_>>().join("\n")
  }
}

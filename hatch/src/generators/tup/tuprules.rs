use platform::arch::Arch;
use project::Project;
use project::ProjectKind;
use project::Target;

fn arch_flag(arch: &Arch) -> String {
    match *arch {
        Arch::X64 => String::from("-m64"),
        Arch::X86 => String::from("-m32"),
    }
}

fn type_flag(lib_type: &ProjectKind) -> String {
    match *lib_type {
        ProjectKind::Static => String::from("-static"),
        ProjectKind::Shared => String::from("-dynamic"),
        _ => String::from(""),
    }
}

pub fn make_tuprules_string(project: &Project) -> String {
    let mut tokens = Vec::new();
    tokens.push(String::from(".gitignore"));

    let project_kind = project.kind();
    let maybe_compiler_options = project.compiler_options();
    let _ = match maybe_compiler_options {
        Some(compiler_options) => {
            let compiler_token = format!("CC = {}", compiler_options.compiler());
            tokens.push(compiler_token);

            match compiler_options.target() {
                Target::Debug => {
                    let debug_token = String::from("CFLAGS += -g");
                    tokens.push(debug_token);
                }
                _ => {}
            }

            let arch_flag = arch_flag(compiler_options.arch());
            let arch_token = format!("ARCH = {}", arch_flag);
            tokens.push(arch_token);
            tokens.push(String::from("CFLAGS += $(ARCH)"));

            if !compiler_options.compiler_flags().is_empty() {
                let compiler_flags = format!("CFLAGS += {}", compiler_options.compiler_flags());
                tokens.push(compiler_flags);
            }

            tokens.push(String::from("LINKFLAGS += $(ARCH)"));

            if !compiler_options.linker_flags().is_empty() {
                let linker_flags = format!("LINKFLAGS += {}", compiler_options.linker_flags());
                tokens.push(linker_flags);
            }
        }
        _ => panic!(""),
    };

    match project_kind {
        ProjectKind::Static | ProjectKind::Shared => {
            let link_flags_type = format!(
                "ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += {}
endif",
                type_flag(project_kind)
            );

            tokens.push(link_flags_type);
        }
        _ => {}
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
include @(TUP_PLATFORM).tup",
    ));

    match project_kind {
        ProjectKind::Static | ProjectKind::Shared => {
            tokens.push(String::from(
                "ifeq ($(LIB_TYPE),static)
  EXTENSION = $(STATIC)
else
  ifeq ($(LIB_TYPE),shared)
    EXTENSION = $(SHARED)
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)",
            ));
        }
        _ => {}
    }

    tokens
        .iter()
        .map(|token| token.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

//pub struct Tuprules {}

//impl Tuprules {
////    pub fn name() -> String {
////        String::from("Tuprules.tup")
////    }
//
//
//}

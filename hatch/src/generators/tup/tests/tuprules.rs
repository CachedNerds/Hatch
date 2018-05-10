use platform::arch::Arch;
use project::CompilerOptions;
use project::Target;
use project::ProjectKind;
use generators::tup::tuprules::make_tuprules_string;
use generators::tup::tests::fixtures::project_with_compiler_options;

/**
I'm sorry Danny. I have completely destroyed your beautiful, elegant, All-pairs testing strategy.
I'm just trying to get things building again. We can fix this!
*/

/**
 * These tests were created using All-Pairs testing.
 *
 * Note that I am not including the Compiler field as it is required and we are simply doing a
 * string replace as opposed to an enumeration.
 *
 * the tests are following this table:
 * (Kind,   Arch,  Target, Compiler Flags, Linker Flags)
 * (Static,  X64, Release,        Present,      Present)
 * (Static,  X86,   Debug,           None,         None)
 * (Shared,  X64,   Debug,           None,         None)
 * (Shared,  X86, Release,        Present,      Present)
 * (Binary,  X64, Release,           None,         None)
 * (Binary,  X86,   Debug,        Present,      Present)
 * (Binary,  X64, Release,        Present,         None)
 * (Binary,  X86,   Debug,           None,      Present)
 */

#[test]
fn build_static_x64_release_with_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
ARCH = -m64
CFLAGS += $(ARCH)
CFLAGS += -c --std=c++1z
LINKFLAGS += $(ARCH)
LINKFLAGS += -v
ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -static
endif
SOURCE = src
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
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)",
    );

  let compiler_options = CompilerOptions::new(
                           String::from("g++"),
                           vec![String::from("-c"), String::from("--std=c++1z")].join(' '.to_string().as_str()),
                           vec![String::from("-v")].join(' '.to_string().as_str()),
                           Arch::X64,
                           Target::Release);

    let project = project_with_compiler_options(ProjectKind::Static, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_static_x86_debug_without_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
CFLAGS += -g
ARCH = -m32
CFLAGS += $(ARCH)
LINKFLAGS += $(ARCH)
ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -static
endif
SOURCE = src
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
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)");

  let compiler_options = CompilerOptions::new(
                           String::from("g++"),
                           String::from(""),
                           String::from(""),
                           Arch::X86,
                           Target::Debug);

    let project = project_with_compiler_options(ProjectKind::Static, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_shared_x64_debug_without_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
CFLAGS += -g
ARCH = -m64
CFLAGS += $(ARCH)
LINKFLAGS += $(ARCH)
ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -dynamic
endif
SOURCE = src
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
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)",
    );

  let compiler_options = CompilerOptions::new(
                           String::from("g++"),
                           String::from(""),
                           String::from(""),
                           Arch::X64,
                           Target::Debug);

    let project = project_with_compiler_options(ProjectKind::Shared, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_shared_x86_release_with_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
ARCH = -m32
CFLAGS += $(ARCH)
CFLAGS += -c --std=c++1z
LINKFLAGS += $(ARCH)
LINKFLAGS += -v
ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -dynamic
endif
SOURCE = src
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
  endif
endif
PROJECT_LIB = $(PROJECT).$(EXTENSION)",
    );

    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        vec![String::from("-c"), String::from("--std=c++1z")].join(' '.to_string().as_str()),
        vec![String::from("-v")].join(' '.to_string().as_str()),
        Arch::X86,
        Target::Release,
    );

    let project = project_with_compiler_options(ProjectKind::Shared, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_binary_x64_release_without_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
ARCH = -m64
CFLAGS += $(ARCH)
LINKFLAGS += $(ARCH)
SOURCE = src
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
    );

    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        String::from(""),
        String::from(""),
        Arch::X64,
        Target::Release,
    );

    let project = project_with_compiler_options(ProjectKind::Binary, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_binary_x86_debug_with_flags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
CFLAGS += -g
ARCH = -m32
CFLAGS += $(ARCH)
CFLAGS += -c --std=c++1z
LINKFLAGS += $(ARCH)
LINKFLAGS += -v
SOURCE = src
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
    );

    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        vec![String::from("-c"), String::from("--std=c++1z")].join(' '.to_string().as_str()),
        vec![String::from("-v")].join(' '.to_string().as_str()),
        Arch::X86,
        Target::Debug,
    );

    let project = project_with_compiler_options(ProjectKind::Binary, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_binary_x64_release_with_cflags_without_lflags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
ARCH = -m64
CFLAGS += $(ARCH)
CFLAGS += -c --std=c++1z
LINKFLAGS += $(ARCH)
SOURCE = src
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
    );

    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        vec![String::from("-c"), String::from("--std=c++1z")].join(' '.to_string().as_str()),
        String::from(""),
        Arch::X64,
        Target::Release,
    );

    let project = project_with_compiler_options(ProjectKind::Binary, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

#[test]
fn build_binary_x86_debug_without_cflags_with_lflags_tuprules() {
    let contents = String::from(
        ".gitignore
CC = g++
CFLAGS += -g
ARCH = -m32
CFLAGS += $(ARCH)
LINKFLAGS += $(ARCH)
LINKFLAGS += -v
SOURCE = src
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
    );

    let compiler_options = CompilerOptions::new(
        String::from("g++"),
        String::from(""),
        vec![String::from("-v")].join(' '.to_string().as_str()),
        Arch::X86,
        Target::Debug,
    );

    let project = project_with_compiler_options(ProjectKind::Binary, compiler_options);
    let actual_contents = make_tuprules_string(&project);

    assert_eq!(contents, actual_contents);
}

use assets::tuprules::Tuprules;
use project::{ ProjectKind, Arch, BuildConfig, Target, LibraryKind };

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
PROJECT_LIB = $(PROJECT).$(EXTENSION)");

  let config = BuildConfig::new(ProjectKind::Library(LibraryKind::Static),
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                vec![String::from("-v")],
                                Arch::X64,
                                Target::Release);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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

  let config = BuildConfig::new(ProjectKind::Library(LibraryKind::Static),
                                String::from("g++"),
                                Vec::new(),
                                Vec::new(),
                                Arch::X86,
                                Target::Debug);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
PROJECT_LIB = $(PROJECT).$(EXTENSION)");

  let config = BuildConfig::new(ProjectKind::Library(LibraryKind::Shared),
                                String::from("g++"),
                                Vec::new(),
                                Vec::new(),
                                Arch::X64,
                                Target::Debug);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
PROJECT_LIB = $(PROJECT).$(EXTENSION)");

  let config = BuildConfig::new(ProjectKind::Library(LibraryKind::Shared),
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                vec![String::from("-v")],
                                Arch::X86,
                                Target::Release);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
include @(TUP_PLATFORM).tup");

  let config = BuildConfig::new(ProjectKind::Binary,
                                String::from("g++"),
                                Vec::new(),
                                Vec::new(),
                                Arch::X64,
                                Target::Release);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
include @(TUP_PLATFORM).tup");

  let config = BuildConfig::new(ProjectKind::Binary,
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                vec![String::from("-v")],
                                Arch::X86,
                                Target::Debug);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
include @(TUP_PLATFORM).tup");

  let config = BuildConfig::new(ProjectKind::Binary,
                                String::from("g++"),
                                vec![String::from("-c"), String::from("--std=c++1z")],
                                Vec::new(),
                                Arch::X64,
                                Target::Release);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
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
include @(TUP_PLATFORM).tup");

  let config = BuildConfig::new(ProjectKind::Binary,
                                String::from("g++"),
                                Vec::new(),
                                vec![String::from("-v")],
                                Arch::X86,
                                Target::Debug);

  let tuprules = Tuprules::new(&config);

  assert_eq!(contents, tuprules.to_string());
}
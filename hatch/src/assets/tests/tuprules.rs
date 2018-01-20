use assets::Arch;
use assets::tuprules::Tuprules;
use project::LibraryKind;

#[test]
fn build_tuprules() {
  let contents = String::from(
".gitignore
CC = g++
ARCH = -m64
CFLAGS += $(ARCH)
CFLAGS += -std=c++1z
CFLAGS += -c
CFLAGS += -I ../../
LINKFLAGS += $(ARCH)
ifneq (@(TUP_PLATFORM),macosx)
  LINKFLAGS += -dynamic
endif
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
PROJECT_LIB = $(PROJECT).$(EXTENSION)");

  let tuprules = Tuprules::new(String::from("g++"), false, Arch::X64, String::from("c++1z"), &LibraryKind::Shared);

  assert_eq!(contents, tuprules.to_string());
}
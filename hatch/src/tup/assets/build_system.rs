pub struct BuildAssets {}

impl BuildAssets {
  pub fn tuprules() -> String {
".gitignore
CC = g++

# Uncomment to build with debug symbols
#CFLAGS += -g

ARCH = -m64
#ARCH = -m32

CFLAGS += $(ARCH)

CFLAGS += -std=c++1z
CFLAGS += -c
CFLAGS += -I ../../

LINKFLAGS += $(ARCH)
LINKFLAGS += -static
LINKFLAGS += -v

SOURCE = src
SOURCE_OUT = build
SOURCE_FILES = $(SOURCE)/*.cpp
SOURCE_OBJ_FILES = $(SOURCE_OUT)/*.o

TEST = test
TEST_OUT = $(TEST)/build
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

PROJECT_LIB = $(PROJECT).$(EXTENSION)".to_string()
  }
}


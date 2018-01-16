use assets::builder::Builder as AssetBuilder;
use assets::ProjectAsset;
use project::{ Project, ProjectKind, LibraryKind };

#[test]
fn build_config_asset() {
  let project = Project::new(String::from("test"), ProjectKind::Library(LibraryKind::Static), String::from("0.1.0"));

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.config(&project);

  let expected_contents = String::from("PROJECT = test\nLIB_TYPE = static");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("config.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_test_tupfile_asset() {
  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.test_tupfile();

  let expected_contents = String::from(".gitignore");
  let expected_asset = ProjectAsset::new(String::from("./test/"), String::from("Tupfile"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_tuprules_asset() {
  let project = Project::new(String::from("test"), ProjectKind::Library(LibraryKind::Shared), String::from("0.1.0"));

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.tuprules(&project);

  let expected_contents = String::from(
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
PROJECT_LIB = $(PROJECT).$(EXTENSION)");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("Tuprules.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_tupfile_asset() {
  let project = Project::new(String::from("test"), ProjectKind::Library(LibraryKind::Static), String::from("0.1.0"));

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.tupfile(&project);

  let expected_contents = String::from(
"include config.tup
include_rules

# override build variables
# VARIABLE = new_value

# define custom build variables

# Compile Source
: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_OUT)/%B.o

# Archive Source
: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_OUT)/$(PROJECT_LIB) ../<$(PROJECT)>

# Compile Tests
: foreach $(TEST_FILES) |> !compile |> $(TEST_OUT)/%B.o

# Create Link Executable
: $(TEST_OBJ_FILES) $(SOURCE_OUT)/$(PROJECT_LIB) |> !link |> $(TEST_OUT)/$(PROJECT).test");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("Tupfile"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_linux_asset() {
  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.linux();

  let expected_contents = String::from("STATIC = a\nSHARED = so");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("linux.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_macos_asset() {
  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.macos();

  let expected_contents = String::from("STATIC = a\nSHARED = so");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("macosx.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_windows_asset() {
  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.windows();

  let expected_contents = String::from(
"STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>");
  let expected_asset = ProjectAsset::new(String::from("./"), String::from("win32.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}
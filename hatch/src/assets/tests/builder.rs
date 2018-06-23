use assets::builder::Builder as AssetBuilder;
use assets::{ Asset, ProjectAsset };
use std::path::PathBuf;
use assets::tests::fixtures;
use project::ProjectKind;

#[test]
fn add_asset() {
  let mut asset_builder = AssetBuilder::new();
  let asset = ProjectAsset::new(PathBuf::from("./"), String::from("test"), String::from("test"));
  asset_builder.add_asset(asset);

  let assets = asset_builder.assets();

  assert_eq!(assets.len(), 1);

  assert_eq!(assets[0].name(), "test");
}

#[test]
fn build_config_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.config(&project);

  let expected_contents = String::from("PROJECT = test\nLIB_TYPE = static");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("config.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_test_tupfile_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.test_tupfile(&project);

  let expected_contents = String::from(".gitignore");
  let expected_asset = ProjectAsset::new(PathBuf::from("./test/"), String::from("Tupfile"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_tuprules_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.tuprules(&project);

  let expected_contents = String::from(
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
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("Tuprules.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_tupfile_asset() {
  let project = fixtures::project(ProjectKind::Shared);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.tupfile(&project);

  let expected_contents = String::from(
"include config.tup
include_rules

: foreach $(SOURCE_FILES) |> !compile |> $(SOURCE_TARGET)/%B.o

: $(SOURCE_OBJ_FILES) |> !archive |> $(SOURCE_TARGET)/$(PROJECT_LIB) <$(PROJECT)>

: foreach $(TEST_FILES) |> !compile |> $(TEST_TARGET)/%B.o

: $(TEST_OBJ_FILES) $(SOURCE_TARGET)/$(PROJECT_LIB) |> !link |> $(TEST_TARGET)/$(PROJECT).test");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("Tupfile"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_tupfile_ini_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.tupfile_ini(&project);

  let expected_contents = String::from("");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("Tupfile.ini"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_linux_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.linux(&project);

  let expected_contents = String::from("STATIC = a\nSHARED = so");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("linux.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_macos_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.macos(&project);

  let expected_contents = String::from("STATIC = a\nSHARED = so");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("macosx.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_windows_asset() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.windows(&project);

  let expected_contents = String::from(
"STATIC = lib
SHARED = dll
# Use clang for front-end
CC = clang++.exe
# Use llvm-lib for static libraries
!archive = |> llvm-lib /MACHINE:X64 /OUT:%o %f |>");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("win32.tup"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

#[test]
fn build_catch_definition() {
  let project = fixtures::project(ProjectKind::Static);

  let asset_builder = AssetBuilder::new();
  let actual_asset = asset_builder.catch_definition(&project);

  let expected_contents = String::from("#define CATCH_CONFIG_MAIN\n#include \"catch.hpp\"");
  let expected_asset = ProjectAsset::new(PathBuf::from("./"), String::from("catch.cpp"), expected_contents);

  assert_eq!(actual_asset, expected_asset);
}

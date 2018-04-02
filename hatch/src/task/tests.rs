use task;
use project::{ Project, ProjectKind };
use project::build::{ Target, Config };
use platform::arch::Arch;
use platform::os;
use assets::PlatformKind;
use std::fs;
use std::path::{ Path, PathBuf };

fn assets_exist(path: &Path) -> bool {
  let file_path = match os::platform_type() {
    PlatformKind::MacOS => path.join("macosx.tup"),
    PlatformKind::Windows => path.join("win32.tup"),
    PlatformKind::Linux => path.join("linux.tup")
  };

  path.join("Tupfile").exists() &&
  path.join("config.tup").exists() &&
  path.join("Tuprules.tup").exists() &&
  path.join("Tupfile.ini").exists() &&
  path.join("test").join("Tupfile").exists() &&
  path.join("test/src").join("catch.hpp").exists() &&
  path.join("test/src").join("catch.cpp").exists() &&
  file_path.exists()
}

fn remove_assets(path: &Path) {
  let _ = fs::remove_dir_all(&path);
}

#[test]
fn generate_assets_success() {
  let path = PathBuf::from("./temp/");
  let _ = fs::create_dir_all(&path);

  let config = Config::new(ProjectKind::Binary,
                           String::from("g++"),
                           vec![String::from("-c"), String::from("--std=c++1z")],
                           vec![String::from("-v")],
                           Arch::X64,
                           Target::Release);

  let project = Project::new(String::from("test"),
                             String::from("0.1.0"),
                             config,
                             Vec::new(),
                             PathBuf::from(&path));

  if let Err(e) = task::generate_assets(&project) {
    remove_assets(&path);
    panic!(e)
  }

  let exists = assets_exist(&path);
  remove_assets(&path);

  assert!(exists);
}

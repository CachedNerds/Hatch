use std::fs;
use std::path;
use std::io::{ Read, Write };

use tup::{ PlatformKind, ProjectKind };
use tup::assets::{ PlatformAssets, BuildAssets, ProjectAssets, TestAssets, Assets };

fn read_file(path: String) -> Option<String>{
  let mut file = String::new();
  match fs::File::open(&path).and_then(|mut f| f.read_to_string(&mut file)) {
    Ok(_) => Some(file),
    Err(_) => None,
  }
}

fn write_file<T: Assets>(asset: &T) {
  let _ = write!(fs::File::create(&asset.path()).unwrap(), "{}", &asset.contents());
}

#[derive(Debug)]
pub struct Manifest {
  platform: Option<PlatformKind>,
  tup_rules: Option<String>,
  project_manifest: ProjectManifest,
}

impl Manifest {
  pub fn new(path: &str, name: &str) -> Manifest {
    let platform = Some(PlatformKind::Linux);

    let tup_rules = read_file(path.to_string() + "/Tuprules.tup");

    let project_manifest = ProjectManifest::new(&path, &name);

    Manifest { platform, tup_rules, project_manifest }
  }

  pub fn project_manifest(&self) -> &ProjectManifest {
    &self.project_manifest
  }
  
  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn create_tuprules(&self, path: &str) {
    let tuprules = BuildAssets::tuprules(&path);
    write_file(&tuprules);
  }

  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn create_linux_platform(&self, path: &str) {
    let linux = PlatformAssets::linux(&path);
    write_file(&linux);
  }

  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn create_darwin_platform(&self, path: &str) {
    let darwin = PlatformAssets::darwin(&path);
    write_file(&darwin);
  }

  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn create_win32_platform(&self, path: &str) {
    let win32 = PlatformAssets::win32(&path);
    write_file(&win32);
  }
}

#[derive(Debug)]
pub struct ProjectManifest {
  config: Option<String>,
  tupfile: Option<String>,
  test_manifest: TestManifest,
}

impl ProjectManifest {
  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn new(path: &str, name: &str) -> ProjectManifest {
    let config = read_file(path.to_string() + "/" + name + "/config.tup");
    let tupfile = read_file(path.to_string() + "/" + name + "/Tupfile");

    let test_manifest = TestManifest::new(&path, &name);

    ProjectManifest { config, tupfile, test_manifest }
  }

  pub fn test_manifest(&self) -> &TestManifest {
    &self.test_manifest
  }

  pub fn create_config(path: &str, name: &str, kind: &ProjectKind) {
    let config = ProjectAssets::config(&path, &name, &kind);
    write_file(&config);
  }

  pub fn create_tupfile(path: &str, name: &str) {
    let tupfile = ProjectAssets::tupfile(&path, &name);
    write_file(&tupfile);
  }
}

#[derive(Debug)]
pub struct TestManifest {
  tupfile: Option<String>,
}

impl TestManifest {
  pub fn new(path: &str, name: &str) -> TestManifest {
    let tupfile = read_file(path.to_string() + "/" + name + "/test/Tupfile");

    TestManifest { tupfile }
  }

  pub fn create_tupfile(path: &str, name: &str) {
    let tupfile = TestAssets::tupfile(&path, &name);
    write_file(&tupfile);
  }
}

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

fn write_file<T: Assets>(asset: T) {
  let _ = write!(fs::File::create(&asset.path()).unwrap(), "{}", &asset.contents());
}


#[derive(Debug)]
pub struct Manifest {
  platform: PlatformKind,
  tup_rules: Option<String>,
  project_manifest: ProjectManifest,
}

impl Manifest {
  pub fn new(path: &str, name: &str) -> Manifest {
    let platform = PlatformKind::Linux;

    let tup_rules = read_file(path.to_string() + "/Tuprules.tup");

    let project_manifest = ProjectManifest::new(&path, &name);

    Manifest { platform, tup_rules, project_manifest }
  }

  pub fn project_manifest(&self) -> &ProjectManifest {
    &self.project_manifest
  }
  
  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn gen_tuprules(&self, path: &str) {
    write_file(BuildAssets::tuprules(&path));
  }
  
  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn gen_linux_platform(&self, path: &str) {
    write_file(PlatformAssets::linux(&path));
  }

  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn gen_darwin_platform(&self, path: &str) {
    write_file(PlatformAssets::darwin(&path));
  }

  // Expects path to be /xxxx/xxxx/Toolbox/C++/libs
  pub fn gen_win32_platform(&self, path: &str) {
    write_file(PlatformAssets::win32(&path));
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

  pub fn gen_config(path: &str, name: &str, kind: &ProjectKind) {
    write_file(ProjectAssets::config(&path, &name, &kind));
  }

  pub fn gen_tupfile(path: &str, name: &str) {
    write_file(ProjectAssets::tupfile(&path, &name));
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

  pub fn gen_tupfile(path: &str, name: &str) {
    write_file(TestAssets::tupfile(&path, &name));
  }
}

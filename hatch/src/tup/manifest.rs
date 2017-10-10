use std::fs;
use std::path;
use std::io::Read;

use tup::{ PlatformKind };

fn read_file(path: &mut path::PathBuf) -> Option<String>{
  let mut file = String::new();
  match fs::File::open(&path).and_then(|mut f| f.read_to_string(&mut file)) {
    Ok(_) => Some(file),
    Err(_) => None,
  }
}

#[derive(Debug)]
pub struct Manifest {
  platform: PlatformKind,
  tup_rules: Option<String>,
  project_manifest: ProjectManifest,
}

impl Manifest {
  pub fn new(mut path: &mut path::PathBuf) -> Manifest {
    let platform = PlatformKind::Linux;

    path.push("Tuprules.tup");
    let tup_rules = read_file(&mut path);

    let project_manifest = ProjectManifest::new(&mut path);

    Manifest { platform, tup_rules, project_manifest }
  }

  pub fn get_project_manifest(&self) -> &ProjectManifest {
    &self.project_manifest
  }
}

#[derive(Debug)]
pub struct ProjectManifest {
  config: Option<String>,
  tupfile: Option<String>,
  test_manifest: TestManifest,
}

impl ProjectManifest {
  pub fn new(mut path: &mut path::PathBuf) -> ProjectManifest {
    path.set_file_name("config.tup");
    let config = read_file(&mut path);

    path.set_file_name("Tupfile");
    let tupfile = read_file(&mut path);

    let test_manifest = TestManifest::new(&mut path);

    ProjectManifest { config, tupfile, test_manifest }
  }

  pub fn get_test_manifest(&self) -> &TestManifest {
    &self.test_manifest
  }
}

#[derive(Debug)]
pub struct TestManifest {
  tupfile: Option<String>,
}

impl TestManifest {
  pub fn new(mut path: &mut path::PathBuf) -> TestManifest {
    path.set_file_name("Tupfile");
    let tupfile = read_file(&mut path);

    TestManifest { tupfile }
  }
}

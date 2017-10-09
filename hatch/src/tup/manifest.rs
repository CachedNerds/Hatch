use tup::{ PlatformKind };

#[derive(Debug)]
pub struct Manifest {
  platform: PlatformKind,
  rules: String,
  project_manifest: ProjectManifest,
}

#[derive(Debug)]
pub struct ProjectManifest {
  config: String,
  tupfile: String,
}

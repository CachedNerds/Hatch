use generators::Generator;
use project::Project;
use assets::builder::Builder;
use platform::os;
use hatch_error::HatchError;
use failure::ResultExt;
use std::path::PathBuf;

pub struct Tup;

impl Tup {
  pub fn new() -> Tup {
    Tup{}
  }
}

impl Generator for Tup {
  fn generate_assets(&self, project_path: PathBuf, project: &Project) -> Result<(), HatchError> {
    let mut builder = Builder::new(project_path, project);
    builder.config();
    builder.test_tupfile();
    builder.tuprules();
    builder.tupfile();
    builder.tupfile_ini();

    let platform_type = os::platform_type();
    builder.platform(&platform_type);

    if let Ok(catch_header) = builder.catch_header() {
      builder.add_asset(catch_header);
    }

    let catch_definition = builder.catch_definition();
    builder.add_asset(catch_definition);
    let assets = Builder::collect_assets(builder);

    for asset in assets {
      asset.write().with_context(|e| {
        format!("Failed to generate asset: `{}` : {}", asset.path().display(), e)
      })?;
    }
    Ok(())
  }
}
use failure::ResultExt;
use generators::tup::builder::Builder;
use generators::Generator;
use hatch_error::HatchError;
use platform::os;
use project::Project;
use std::path::PathBuf;

pub struct Tup;

impl Tup {
    pub fn new() -> Tup {
        Tup {}
    }
}

impl Generator for Tup {
    fn generate_assets(&self, project_path: PathBuf, project: &Project) -> Result<(), HatchError> {
        let mut builder = Builder::new(project_path, project);
        builder.add_tup_config();
        builder.add_test_tupfile();
        builder.add_tuprules();
        builder.add_tupfile();
        builder.add_tupfile_ini();

        let platform_type = os::platform_type();
        builder.platform(&platform_type);

        if let Ok(catch_header) = builder.add_catch_header() {
            builder.add_asset(catch_header);
        }

        let catch_definition = builder.add_catch_definition();
        builder.add_asset(catch_definition);
        let assets = builder.collect_assets();

        for asset in assets {
            asset.write().with_context(|e| {
                format!(
                    "Failed to generate asset: `{}` : {}",
                    asset.path().display(),
                    e
                )
            })?;
        }
        Ok(())
    }
}

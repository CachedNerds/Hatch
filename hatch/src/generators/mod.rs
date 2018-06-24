use hatch_error::HatchResult;
use project::Project;
use std::path::PathBuf;

pub mod tup;

pub mod platform_kind;
pub mod project_asset;

pub trait Generator {
    // TODO: this interface should take references
    fn generate_assets(&self, project_path: PathBuf, project: &Project) -> HatchResult<()>;
}

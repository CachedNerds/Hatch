use assets::ProjectAsset;
use hatch_error::{HatchResult, ResultExt};

pub fn generate_all(assets: &Vec<ProjectAsset>) -> HatchResult<()> {
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

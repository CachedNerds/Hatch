use failure::ResultExt;
use generators::project_asset::ProjectAsset;
use generators::tup::tup::Tup;
use generators::Generator;
use hatch_error::HatchResult;

pub mod tup;

pub mod builder;
pub mod catch_definition;
pub mod catch_header;
pub mod platform;
pub mod test_tupfile;
pub mod tup_config;
pub mod tup_kind;
pub mod tupfile;
pub mod tupfile_ini;
pub mod tuprules;

#[cfg(test)]
mod tests;

pub fn make_a_tup_in_a_box() -> Box<Generator> {
    let generator: Box<Generator> = Box::new(Tup {});
    generator
}

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

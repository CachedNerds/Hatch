use generators::tup::tup::Tup;
use generators::Generator;
use hatch_error::HatchResult;
use failure::ResultExt;
use generators::project_asset::ProjectAsset;

pub mod tup;

pub mod builder;
pub mod catch_definition;
pub mod catch_header;
pub mod tupfile;
pub mod tupfile_ini;
pub mod test_tupfile;
pub mod tuprules;
pub mod tup_config;
pub mod platform;
pub mod tup_kind;

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
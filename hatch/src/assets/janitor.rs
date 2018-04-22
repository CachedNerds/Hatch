use assets::{Asset, ProjectAsset};
use hatch_error::{HatchResult, ResultExt};
use std::path::PathBuf;
use std::fs;

fn remove_file(file_path: &PathBuf) -> HatchResult<()> {
    lsfs::remove_file(&file_path)?;

    Ok(())
}

fn remove_dir_all(dir_path: &PathBuf) -> HatchResult<()> {
    fs::remove_dir_all(&dir_path)?;

    Ok(())
}

pub fn remove_asset(asset: &ProjectAsset) -> HatchResult<()> {
    let asset_path = asset.path().join(asset.name());
    remove_file(&asset_path)
}

pub fn remove_assets(assets: &Vec<ProjectAsset>) -> Vec<HatchResult<()>> {
    assets
        .iter()
        .map(|asset| remove_asset(asset))
        .collect::<_>()
}

pub fn remove_targets(project_path: &PathBuf) -> Vec<HatchResult<()>> {
    let targets = vec!["target", "test/target"];
    targets
        .iter()
        .map(|target| remove_dir_all(&project_path.join(target)))
        .collect::<_>()
}

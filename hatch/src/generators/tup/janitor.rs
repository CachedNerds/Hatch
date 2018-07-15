use hatch_error::{HatchResult};
use std::path::PathBuf;
use std::fs;
use generators::project_asset::ProjectAsset;

fn remove_file(file_path: &PathBuf) -> HatchResult<()> {
  fs::remove_file(&file_path)?;

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
  assets.iter().map(|asset| remove_asset(asset)).collect::<_>()
}

pub fn remove_targets(project_path: &PathBuf) -> Vec<HatchResult<()>> {
  let targets = vec!["target", "test/target"];
  targets.iter().map(|target| remove_dir_all(&project_path.join(target))).collect::<_>()
}

#[cfg(test)]
mod tests {
  use std::fs;
  use assert_fs::TempDir;
  use assert_fs::prelude::*;
  use generators::project_asset::ProjectAsset;
  use generators::tup::janitor::remove_assets;
  use std::path::PathBuf;
  use std::fs::File;

  #[test]
  fn it_removes_assets_and_leaves_everything_else() {

    fn make_sub_dir_asset(temp_dir: PathBuf, dir: &'static str, file: &'static str) -> ProjectAsset {
      let mut dir_path = temp_dir;
      dir_path.push(dir);
      let _ = fs::create_dir(dir_path.as_path());
      let mut file_path = dir_path.clone();
      file_path.push(file);
      let _ = File::create(file_path.as_path());
      let asset = ProjectAsset::new(
        file_path,
        file.to_string(),
        "".to_string());
      asset
    }

    let temp = TempDir::new().unwrap();
    let temp_dir = temp.path().to_path_buf();
    // top level file
    let foo_file = temp.child("foo.txt");
    foo_file.touch().unwrap();
    let foo_asset = ProjectAsset::new(
      foo_file.path().to_path_buf(),
      "foo".to_string(),
      "foo stuff".to_string());
    // a nested file
    let bar_asset = make_sub_dir_asset(temp_dir.to_path_buf(), "bar", "bar.txt");
    // a file that is NOT an asset
    let baz_asset = make_sub_dir_asset(temp_dir.to_path_buf(), "baz", "baz.txt");

    let assets = vec![foo_asset.clone(), bar_asset.clone()];
    let result = remove_assets(&assets);
    assert_eq!(result.len(), 2);
    assert!(!foo_file.path().exists(), "Foo asset was not deleted");
    assert!(!bar_asset.path().exists(), "Bar asset was not deleted");
    assert!(baz_asset.path().exists(), "Baz asset was incorrectly deleted");
  }
}


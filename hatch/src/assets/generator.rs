use assets::{ Asset, ProjectAsset };
use hatch_error::{ HatchResult, ResultExt };
use std::fs;
use std::io::prelude::*;
use std::io::Write;

pub fn generate_one(asset: &ProjectAsset) -> HatchResult<()> {
  let path = asset.path();
  fs::create_dir_all(&path)?;

  let file_path = path.join(asset.name());
  let mut file = fs::File::create(&file_path)?;

  let contents = asset.contents();
  file.write_all(contents.as_bytes())?;

  Ok(())
}

pub fn generate_all(assets: &Vec<ProjectAsset>) -> HatchResult<()> {
  for asset in assets {
    generate_one(asset)?;
  }

  Ok(())
}
use assets::{ ProjectAsset };
use hatch_error::{ HatchResult, ResultExt };
use std::fs;
use std::io::Write;

//pub fn generate_one(asset: &ProjectAsset) -> HatchResult<()> {
//  let path = asset.path();
//  fs::create_dir_all(&path).with_context(|e| {
//    format!("Failed to create directory: `{}` : {}", path.display(), e)
//  })?;
//
//  let file_path = path.join(asset.name());
//  let mut file = fs::File::create(&file_path).with_context(|e| {
//    format!("Failed to create file: `{}` : {}", file_path.display(), e)
//  })?;
//
//  file.write_all(asset.contents.as_bytes()).with_context(|e| {
//    format!("Failed to write contents to file: `{}` : {}", file_path.display(), e)
//  })?;
//
//  Ok(())
//}

pub fn generate_all(assets: &Vec<ProjectAsset>) -> HatchResult<()> {
  for asset in assets {
    asset.write().with_context(|e| {
      format!("Failed to generate asset: `{}` : {}", asset.path().display(), e)
    })?;
  }

  Ok(())
}
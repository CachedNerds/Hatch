use assets::{ Asset, ProjectAsset };
use std::fs;
use std::io::Write;

pub fn generate_one(asset: &ProjectAsset) {
  let file_path = asset.file_path();
  let contents = asset.contents();

  match fs::File::create(file_path) {
    Ok(mut file) => {
      file.write_all(contents.as_bytes());
    },
    _ => {}
  };
}

pub fn generate_all(assets: &Vec<ProjectAsset>) {
  for asset in assets {
    generate_one(asset);
  }
}
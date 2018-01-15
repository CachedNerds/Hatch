use assets::{ Asset, ProjectAsset };
use std::fs;
use std::io::Write;

pub fn generate_one(asset: &ProjectAsset) {
  let path = asset.path();

  match fs::create_dir_all(&path) {
    Err(e) => println!("{:?}", e),
    Ok(_) => {
      let file_path = String::from(path) + asset.name();
      match fs::File::create(&file_path) {
        Ok(mut file) => {
          let contents = asset.contents();
          file.write_all(contents.as_bytes());
        },
        Err(e) => {
          println!("{:?}", e);
        }
      }
    }
  }
}

pub fn generate_all(assets: &Vec<ProjectAsset>) {
  for asset in assets {
    generate_one(asset);
  }
}
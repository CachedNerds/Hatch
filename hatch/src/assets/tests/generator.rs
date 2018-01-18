use assets::generator::*;
use assets::ProjectAsset;

use std::fs;
use std::io::prelude::*;

#[test]
fn generate_one_without_directories() {
  let test_asset = ProjectAsset::new(String::from("./"), String::from("test.test"), String::from("test"));

  generate_one(&test_asset);

  match fs::File::open("./test.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "test");
      
      fs::remove_file("./test.test");
    },
    Err(e) => panic!(e)
  }
}

#[test]
fn generate_one_with_directories() {
  let test_asset = ProjectAsset::new(String::from("./test/"), String::from("test.test"), String::from("test"));

  generate_one(&test_asset);

  match fs::File::open("./test/test.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "test");
      
      fs::remove_file("./test/test.test");
      fs::remove_dir("./test/");
    },
    Err(e) => panic!(e)
  }
}

#[test]
fn generate_one_overwrites_file() {
  let test_asset = ProjectAsset::new(String::from("./"), String::from("test2.test"), String::from("old"));

  generate_one(&test_asset);

  // verify that the old content is there
  match fs::File::open("./test2.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "old");
    },
    Err(e) => panic!(e)
  }

  let test_asset = ProjectAsset::new(String::from("./"), String::from("test2.test"), String::from("new"));

  generate_one(&test_asset);

  // verify that the new content overwrites the old content
  match fs::File::open("./test2.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "new");
      
      fs::remove_file("./test2.test");
    },
    Err(e) => panic!(e)
  }
}

#[test]
fn generate_all_assets() {
  let test_asset_one = ProjectAsset::new(String::from("./"), String::from("one.test"), String::from("one"));
  let test_asset_two = ProjectAsset::new(String::from("./"), String::from("two.test"), String::from("two"));

  let test_assets = vec![test_asset_one, test_asset_two];

  generate_all(&test_assets);

  match fs::File::open("./one.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "one");
      
      fs::remove_file("./one.test");
    },
    Err(e) => panic!(e)
  }

    match fs::File::open("./two.test") {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
    
      assert_eq!(contents, "two");
      
      fs::remove_file("./two.test");
    },
    Err(e) => panic!(e)
  }
}
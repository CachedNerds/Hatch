use assets::ProjectAsset;
use assets::generator::*;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

#[test]
fn generate_one_without_directories() {
    let test_asset = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("test.test"),
        String::from("test"),
    );

    if let Err(e) = generate_one(&test_asset) {
        panic!(e);
    }

    match fs::File::open("./test.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./test.test");

            if let Err(e) = result {
                panic!(e)
            }

            assert_eq!(contents, "test");
        }
        Err(e) => {
            fs::remove_file("./test.test");
            panic!(e)
        }
    }
}

#[test]
fn generate_one_with_directories() {
    let test_asset = ProjectAsset::new(
        PathBuf::from("./foo/"),
        String::from("test.test"),
        String::from("test"),
    );

    if let Err(e) = generate_one(&test_asset) {
        panic!(e);
    }

    match fs::File::open("./foo/test.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./foo/test.test");
            fs::remove_dir("./foo/");

            if let Err(e) = result {
                panic!(e)
            }
        }
        Err(e) => {
            fs::remove_file("./foo/test.test");
            fs::remove_dir("./foo/");
            panic!(e)
        }
    }
}

#[test]
fn generate_one_overwrites_file() {
    let test_asset = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("test2.test"),
        String::from("old"),
    );

    if let Err(e) = generate_one(&test_asset) {
        panic!(e);
    }

    // verify that the old content is there
    match fs::File::open("./test2.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./test2.test");

            if let Err(e) = result {
                panic!(e)
            }

            assert_eq!(contents, "old");
        }
        Err(e) => {
            fs::remove_file("./test2.test");
            panic!(e)
        }
    }

    let test_asset = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("test2.test"),
        String::from("new"),
    );

    if let Err(e) = generate_one(&test_asset) {
        fs::remove_file("./test2.test");
        panic!(e);
    }

    // verify that the new content overwrites the old content
    match fs::File::open("./test2.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./test2.test");

            if let Err(e) = result {
                panic!(e)
            }

            assert_eq!(contents, "new");
        }
        Err(e) => {
            fs::remove_file("./test2.test");
            panic!(e)
        }
    }
}

#[test]
fn generate_all_assets() {
    let test_asset_one = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("one.test"),
        String::from("one"),
    );
    let test_asset_two = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("two.test"),
        String::from("two"),
    );

    let test_assets = vec![test_asset_one, test_asset_two];

    if let Err(e) = generate_all(&test_assets) {
        fs::remove_file("./one.test");
        fs::remove_file("./two.test");
        panic!(e);
    }

    match fs::File::open("./one.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./one.test");

            if let Err(e) = result {
                fs::remove_file("./two.test");
                panic!(e)
            }

            assert_eq!(contents, "one");
        }
        Err(e) => {
            fs::remove_file("./one.test");
            fs::remove_file("./two.test");
            panic!(e)
        }
    }

    match fs::File::open("./two.test") {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file.read_to_string(&mut contents);
            fs::remove_file("./two.test");

            if let Err(e) = result {
                panic!(e)
            }

            assert_eq!(contents, "two");
        }
        Err(e) => {
            fs::remove_file("./two.test");
            panic!(e)
        }
    }
}

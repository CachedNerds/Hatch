mod builder;
mod catch_definition;
mod config;
mod fixtures;
mod generator;
mod platform;
mod test_tupfile;
mod tupfile;
mod tuprules;

use generators::project_asset::ProjectAsset;
use std::path::PathBuf;

#[test]
fn fmt_project_asset() {
    let asset = ProjectAsset::new(
        PathBuf::from("./"),
        String::from("test"),
        String::from("test"),
    );

    let result = format!("{:?}", asset);
    assert_eq!(result, "path: ./, name: test, contents: test");
}

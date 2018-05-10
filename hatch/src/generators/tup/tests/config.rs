use project::{ProjectKind};
use generators::tup::tup_config::make_tup_config_string;

#[test]
fn build_shared_config() {
    let config = make_tup_config_string("Test", &ProjectKind::Shared);

    assert_eq!("PROJECT = Test\nLIB_TYPE = shared", config.to_string());
}

#[test]
fn build_static_config() {
    let config = make_tup_config_string("Test", &ProjectKind::Static);

    assert_eq!("PROJECT = Test\nLIB_TYPE = static", config.to_string());
}

#[test]
fn build_binary_config() {
    let config = make_tup_config_string("Test", &ProjectKind::Binary);

    assert_eq!("PROJECT = Test\nLIB_TYPE = binary", config.to_string());
}

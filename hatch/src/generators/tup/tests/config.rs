use assets::config::Config;
use project::ProjectKind;

#[test]
fn build_shared_config() {
    let config = Config::new("Test", &ProjectKind::Shared);

    assert_eq!("PROJECT = Test\nLIB_TYPE = shared", config.to_string());
}

#[test]
fn build_static_config() {
    let config = Config::new("Test", &ProjectKind::Static);

    assert_eq!("PROJECT = Test\nLIB_TYPE = static", config.to_string());
}

#[test]
fn build_binary_config() {
    let config = Config::new("Test", &ProjectKind::Binary);

    assert_eq!("PROJECT = Test\nLIB_TYPE = binary", config.to_string());
}

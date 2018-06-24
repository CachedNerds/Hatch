use generators::tup::tup_config;
use project::ProjectKind;

#[test]
fn tup_config_file_name() {
    assert_eq!("config.tup", tup_config::file_name());
}

#[test]
fn generate_shared_config() {
    let config = tup_config::make_string("Test", &ProjectKind::Shared);

    assert_eq!("PROJECT = Test\nLIB_TYPE = shared", config);
}

#[test]
fn generate_static_config() {
    let config = tup_config::make_string("Test", &ProjectKind::Static);

    assert_eq!("PROJECT = Test\nLIB_TYPE = static", config);
}

#[test]
fn generate_binary_config() {
    let config = tup_config::make_string("Test", &ProjectKind::Binary);

    assert_eq!("PROJECT = Test\nLIB_TYPE = binary", config);
}

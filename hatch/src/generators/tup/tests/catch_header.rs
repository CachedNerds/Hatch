use generators::tup::catch_header;

#[test]
fn catch_header_file_name() {
    assert_eq!("catch.hpp", catch_header::file_name());
}

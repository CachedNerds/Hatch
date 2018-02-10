use assets::catch_header::CatchHeader;

#[test]
fn build_catch_header() {
  assert_eq!("catch.hpp", CatchHeader::name());
}

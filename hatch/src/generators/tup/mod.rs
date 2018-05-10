use generators::tup::tup::Tup;
use generators::Generator;

pub mod tup;

pub mod builder;
pub mod catch_definition;
pub mod catch_header;
pub mod tupfile;
pub mod tupfile_ini;
pub mod test_tupfile;
pub mod tuprules;
pub mod tup_config;
pub mod platform;

pub fn make_a_tup_in_a_box() -> Box<Generator> {
    let generator: Box<Generator> = Box::new(Tup {});
    generator
}
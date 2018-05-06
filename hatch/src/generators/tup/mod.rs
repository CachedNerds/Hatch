use generators::tup::tup::Tup;
use generators::Generator;

pub mod tup;

mod builder;
mod catch_definition;
mod catch_header;
mod tupfile;
mod tupfile_ini;
mod test_tupfile;
mod tuprules;
mod tup_config;
mod platform;

pub fn make_a_tup_in_a_box() -> Box<Generator> {
    let generator: Box<Generator> = Box::new(Tup {});
    generator
}
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
  let matches = App::new("Hatch")
    .subcommand(SubCommand::with_name("new")
                .arg(Arg::with_name("PROJECT_NAME")
                     .required(true)
                     .takes_value(true)))
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("new") {
    if matches.is_present("PROJECT_NAME") {
      println!("Project name: {}", matches.value_of("PROJECT_NAME").unwrap());
    }
  }
}

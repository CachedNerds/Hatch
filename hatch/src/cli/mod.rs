extern crate clap;
use clap::{Arg, App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
  let new_command = SubCommand::with_name("new")
    .about("Creates a new project")
    
    .arg(Arg::with_name("PROJECT_NAME")
         .help("Name of the project to generate")
         .required(true)
         .takes_value(true))
    
    .arg(Arg::with_name("bin")
         .help("Generates a binary project")
         .long("bin")
         .short("b")
         .required(false))

    .arg(Arg::with_name("static")
         .help("Generate a static library")
         .long("static")
         .short("st")
         .required_unless("bin")
         .conflicts_with_all(&["bin", "shared"]))

    .arg(Arg::with_name("shared")
         .help("Generate a shared library")
         .long("shared")
         .short("sh")
         .required_unless("bin")
         .conflicts_with_all(&["bin", "static"]));

    App::new("hatch")
      .subcommand(new_command)
}

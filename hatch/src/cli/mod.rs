extern crate clap;
use clap::{Arg, App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
  let new_command = SubCommand::with_name("new")
    .about("Creates a new project.
           \nWithout any extra parameters hatch generates a shared library.")
    
    .arg(Arg::with_name("PROJECT_NAME")
         .help("Name of the project to generate")
         .required(true)
         .takes_value(true))
    
    .arg(Arg::with_name("bin")
         .help("Generate a binary project")
         .long("bin")
         .short("b")
         .required(false))

    .arg(Arg::with_name("static")
         .help("Generate a static library")
         .long("static")
         .short("s")
         .conflicts_with("bin"));

  let update_command = SubCommand::with_name("update")
    .about("Updates an existing project");

  App::new("hatch")
    .subcommand(new_command)
    .subcommand(update_command)
}

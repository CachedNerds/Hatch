extern crate clap;
use clap::{ Arg, App, SubCommand, AppSettings };

pub fn build_cli() -> App<'static, 'static> {
  let init_command = SubCommand::with_name("init")
    .about("Generates Tup build system files");

  let new_command = SubCommand::with_name("new")
    .about("Creates a new project.
           \nWithout any extra parameters hatch generates a shared library.")
    
    .arg(Arg::with_name("TOOLBOX_PATH")
         .help("Path to Toolbox.  (Default = cwd)")
         .long("path")
         .short("p")
         .required(false)
         .takes_value(true))

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
    .about("Updates an existing project")

    .arg(Arg::with_name("PROJECT_NAME")
         .help("Name of the project to update")
         .required(true)
         .takes_value(true))

    .arg(Arg::with_name("TOOLBOX_PATH")
         .help("Path to Toolbox.  (Default = cwd)")
         .long("path")
         .short("p")
         .required(false)
         .takes_value(true));

  App::new("hatch")
    .subcommand(new_command)
    .subcommand(update_command)
    .setting(AppSettings::SubcommandRequiredElseHelp)
}

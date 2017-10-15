use project::{ ProjectKind, LibraryKind };
use clap::{ App, SubCommand, Arg, AppSettings, ArgMatches };

pub struct Cli<'cli>(ArgMatches<'cli>);

impl<'cli> Cli<'cli> {
  pub fn new() -> Cli<'cli> {
    Cli(build_cli())
  }

  pub fn build_type(&mut self) -> ProjectKind {
    let arg = self.0.subcommand_matches("new").unwrap();

    if arg.is_present("bin") {
      ProjectKind::Binary
    } else if arg.is_present("static") {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }

  pub fn name(&mut self) -> String {
    let arg = self.0.subcommand_matches("new").unwrap();
    let name = value_t!(arg, "PROJECT_NAME", String).unwrap();
    name
  }

  pub fn path(&mut self) -> String {
    let arg = self.0.subcommand_matches("new").unwrap();
    let mut path = String::new();

    if arg.is_present("TOOLBOX_PATH") {
      path.push_str(value_t!(arg, "TOOLBOX_PATH", String).unwrap().as_str());
    } else {
      path.push_str("./");
    }

    path.push_str("C++/libs");
    path
  }
}

fn build_cli<'cli>() -> ArgMatches<'cli> {
  let new_command = SubCommand::with_name("new")
    .about("Creates a new project. (default = shared library)")
    
    .arg(Arg::with_name("TOOLBOX_PATH")
         .help("Path to toolbox. (default = ./)")
         .long("path")
         .short("p")
         .required(false)
         .takes_value(true))

    .arg(Arg::with_name("PROJECT_NAME")
         .help("Name of project")
         .required(true)
         .takes_value(true))
    
    .arg(Arg::with_name("bin")
         .help("Generate a stand alone executable")
         .long("bin")
         .short("b")
         .required(false))
    
    .arg(Arg::with_name("static")
         .help("Generate a static library")
         .long("static")
         .short("s")
         .conflicts_with("bin")
         .required(false));

  let app = App::new("hatch")
    .subcommand(new_command)
    .setting(AppSettings::SubcommandRequiredElseHelp);

  app.get_matches()
}

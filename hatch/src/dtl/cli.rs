use project::{ ProjectKind, LibraryKind };
use clap::{ App, AppSettings, ArgMatches };

pub struct Cli<'cli>(ArgMatches<'cli>);

impl<'cli> Cli<'cli> {
  pub fn new<I>(subcommands: I) -> Cli<'cli>
    where I: IntoIterator<Item=App<'cli, 'cli>> { 
    Cli(App::new("hatch")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(subcommands)
        .get_matches())
  }

  pub fn subcommand(&self) -> (&str, Option<&ArgMatches<'cli>>) {
    self.0.subcommand()
  }

  pub fn build_type(&self) -> ProjectKind {
    let arg = self.0.subcommand_matches("new").unwrap();

    if arg.is_present("bin") {
      ProjectKind::Binary
    } else if arg.is_present("static") {
      ProjectKind::Library(LibraryKind::Static)
    } else {
      ProjectKind::Library(LibraryKind::Shared)
    }
  }

  pub fn name(&self) -> String {
    let arg = self.0.subcommand_matches("new").unwrap();
    let name = value_t!(arg, "PROJECT_NAME", String).unwrap();
    name
  }

  pub fn path(&self) -> String {
    let arg = self.0.subcommand_matches("new").unwrap();
    let mut path = String::new();

    if arg.is_present("TOOLBOX_PATH") {
      path.push_str(value_t!(arg, "TOOLBOX_PATH", String).unwrap().as_str());
    } else {
      path.push_str("./");
    }

    match path.as_str().chars().last().unwrap() {
      '/' => path,
      _   => {
        path.push_str("/");
        path
      }
    }
  }
}

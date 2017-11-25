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
}

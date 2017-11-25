use clap::{ App, AppSettings, ArgMatches, Arg };

pub struct Cli<'cli>(ArgMatches<'cli>);

impl<'cli> Cli<'cli> {
  pub fn new<I>(subcommands: I) -> Cli<'cli>
    where I: IntoIterator<Item=App<'cli, 'cli>> { 
    Cli(App::new("hatch")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(subcommands.into_iter().map(|s| {
          s.arg(Arg::with_name("TOOLBOX_PATH")
                .help("Path to toolbox. (default = ./)")
                .long("path").short("p").required(false).takes_value(true))

        }).collect::<Vec<_>>())
        .get_matches())
  }

  pub fn subcommand(&self) -> (&str, Option<&ArgMatches<'cli>>) {
    self.0.subcommand()
  }
}

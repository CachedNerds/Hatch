use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;

pub struct Update {
  name: &'static str
}

impl Update {
  pub fn new() -> Update {
    Update { name: "update" }
  }
}

impl<'command> Command<'command> for Update {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Updates project dependencies.")
      .version("0.1.0")
      .author("Mackenzie Clark <mackenzie.a.z.c@gmail.com>")
      
      .arg(Arg::with_name("PROJECT_NAME")
           .help("Name of project")
           .required(false)
           .takes_value(true))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) {
    let mut project_string = String::from("Project Name: ");
    let mut toolbox_string = String::from("Toolbox Path: ");

    if let Some(name) = self.project_name(args) {
      project_string.push_str(name.as_str());
    } else {
      project_string.push_str("Unspecified");
    }

    toolbox_string.push_str(self.toolbox_path(args).as_str());

    println!("{}\n{}", project_string, toolbox_string);
  }
}

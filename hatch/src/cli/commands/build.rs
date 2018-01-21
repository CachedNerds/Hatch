use hatch_error::HatchResult;
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use cli::commands::PROJECT_NAMES;
use project::Project;
use task;
use std::process;

pub struct Build {
  name: &'static str
}

impl<'build> Build {
  pub fn new() -> Build {
    Build {
      name: "build"
    }
  }
}

impl<'command> Command<'command> for Build {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Builds a project.")
      .author("Josh Gould <mrgould93@gmail.com>")

      .arg(Arg::with_name(PROJECT_NAMES)
           .help("The projects to be built.")
           .required(false)
           .min_values(0)
           .value_delimiter(" "))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<Project> {
    let project = task::read_project(self.project_path(args))?;
    task::generate_assets(&project)?;

    if let Some(path) = project.path().to_str() {
      let command = String::from("cd ") + path + " && tup";
      let mut child = process::Command::new("sh")
                                        .arg("-c")
                                        .arg(command)
                                        .spawn()
                                        .expect("failed to build project.");
      child.wait();
    }


    Ok(project)
  }
}

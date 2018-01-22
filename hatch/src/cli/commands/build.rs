use hatch_error::{ HatchResult, ResultExt };
use clap::{ App, SubCommand, Arg, ArgMatches };
use cli::commands::Command;
use cli::commands::PROJECT_NAMES;
use project::Project;
use assets::PlatformKind;
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
    let project_path = self.project_path(args);
    let project = task::read_project(&project_path).with_context(|e| {
      format!("Failed to read project `{:?}` : {}", project_path, e)
    })?;

    task::generate_assets(&project).with_context(|e| {
      format!("Failed to generate assets : {}", e)
    })?;

    if let Some(path) = project.path().to_str() {
      let command = format!("cd {} && tup", path);
      match task::platform_type() {
        PlatformKind::Windows => {
          let mut child = process::Command::new("cmd")
                                            .arg("/C")
                                            .arg(command)
                                            .spawn()
                                            .expect("failed to build project.");
          child.wait();
        },
        _ => {
          let mut child = process::Command::new("sh")
                                            .arg("-c")
                                            .arg(command)
                                            .spawn()
                                            .expect("failed to build project.");
          child.wait();
        }
      }
    }

    Ok(project)
  }
}

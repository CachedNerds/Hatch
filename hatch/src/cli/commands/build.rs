use hatch_error::{ HatchResult, ResultExt, InvalidPathError };
use clap::{ App, SubCommand, ArgMatches };
use cli::commands::Command;
use platform::os;
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

  pub fn execute(&self, project: &Project) -> HatchResult<()> {
    if let Some(path) = project.path().to_str() {
      let command = format!("cd {} && tup", path);
      let mut shell: String;
      let mut args: Vec<String>;
      match os::platform_type() {
        PlatformKind::Windows => {
          shell = String::from("cmd");
          args = vec![String::from("/C"), command];
        },
        _ => {
          shell = String::from("sh");
          args = vec![String::from("-c"), command];
        }
      }

      let mut child = process::Command::new(shell).args(args).spawn()?;
      child.wait()?;

      Ok(())
    } else {
      Err(InvalidPathError)?
    }
  }
}

impl<'command> Command<'command> for Build {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Builds a project.")
      .author("Josh Gould <mrgould93@gmail.com>")
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let res = (|| -> HatchResult<()> {
      let project_path = self.project_path(args);
      let project = task::read_project(&project_path)?;

      println!("Generating assets...\n");

      task::generate_assets(&project)?;

      println!("Building project...\n");

      self.execute(&project)
    })().with_context(|e| {
      format!("Failed to build project : {}", e)
    })?;

    Ok(res)
  }
}

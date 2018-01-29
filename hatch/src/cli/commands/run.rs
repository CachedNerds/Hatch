use std::process::Command as ProcessCommand;
use cli::commands::Command;
use cli::commands::build::Build;
use cli::commands::ARGS;
use hatch_error::{ HatchResult, ResultExt, NullError };
use task;
use clap::{ App, SubCommand, Arg, ArgMatches };
use project::ProjectKind;

pub struct Run {
  name: &'static str,
}

impl<'run> Run {
  pub fn new() -> Run {
    Run {
      name: "run",
    }
  }
}

fn parse_run_arguments_from_cli<'command>(cli_args: &ArgMatches<'command>) -> Vec<String> {
  if let Some(arguments) = cli_args.values_of(ARGS) {
    arguments.map(String::from).collect()
  } else {
    Vec::new()
  }
}

impl<'command> Command<'command> for Run {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Executes a project.")
      .author("Danny Peck <danieljpeck93@gmail.com>")

      .arg(Arg::with_name(ARGS)
        .help("The arguments forwarded to the executable.")
        .min_values(0).value_delimiter(" ")
        .required(false))
  }

  fn subcommand_name(&self) -> &'static str {
    self.name
  }

  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let res = (|| -> HatchResult<()> {
      let project_path = self.project_path(args);
      let project = task::read_project(&project_path)?;

      match *project.kind() {
        ProjectKind::Binary => {
          println!("Generating assets...\n");

          task::generate_assets(&project)?;

          println!("Building project...\n");

          Build::new().execute(&project).with_context(|e| {
            format!("Failed to build project : {}", e)
          })?;

          println!("Executing...\n");

          let executable_path = format!("{}target/{}", project_path.display(), project.name());
          let run_arguments = parse_run_arguments_from_cli(args);

          let mut child = ProcessCommand::new(&executable_path).args(run_arguments).spawn()?;
          child.wait()?;

          Ok(())
        },
        _ => {
          Err(NullError).with_context(|_e| {
            format!("Project must be a Binary project.")
          })?
        }
      }
    })().with_context(|e| {
      format!("Execution has failed : {}", e)
    })?;

    Ok(res)
  }
}

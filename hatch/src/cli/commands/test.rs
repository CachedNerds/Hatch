use std::process::Command as ProcessCommand;
use cli::commands::Command;
use cli::commands::build::Build;
use cli::commands::ARGS;
use hatch_error::{ HatchResult, ResultExt };
use task;
use clap::{ App, SubCommand, Arg, ArgMatches };

pub struct Test {
  name: &'static str,
}

impl<'test> Test {
  pub fn new() -> Test {
    Test {
      name: "test",
    }
  }
}

fn parse_test_arguments_from_cli<'command>(cli_args: &ArgMatches<'command>) -> Vec<String> {
  if let Some(arguments) = cli_args.values_of(ARGS) {
    arguments.map(String::from).collect()
  } else {
    Vec::new()
  }
}

impl<'command> Command<'command> for Test {
  fn cli_subcommand(&self) -> App<'command, 'command> {
    SubCommand::with_name(&self.name)
      .about("Tests a project.")
      .author("Danny Peck <danieljpeck93@gmail.com>")

      .arg(Arg::with_name(ARGS)
        .help("The arguments forwarded to the test executable.")
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

      println!("Generating assets...\n");

      task::generate_assets(&project)?;

      println!("Building project...\n");

      Build::new().execute(&project_path, &project).with_context(|e| {
        format!("Failed to build project : {}", e)
      })?;

      println!("Executing tests...\n");

      let test_executable_path = format!("{}test/target/{}.test", project_path.display(), project.name());
      let test_arguments = parse_test_arguments_from_cli(args);

      let mut child = ProcessCommand::new(&test_executable_path).args(test_arguments).spawn()?;
      child.wait()?;

      Ok(())
    })().with_context(|e| {
      format!("Test execution has failed : {}", e)
    })?;

    Ok(res)
  }
}

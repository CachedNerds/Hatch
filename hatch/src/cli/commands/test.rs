use std::process::Command as ProcessCommand;
use cli::commands::Command;
use hatch_error::{ HatchResult, ResultExt };
use task;
use clap::{ ArgMatches };

pub struct Test;

impl<'test> Test {
  pub fn new() -> Test {
    Test
  }
}


impl<'command> Command<'command> for Test {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let project_path = self.project_path(args);
    let project = task::read_project(&project_path)?;

    println!("Generating assets...\n");

    task::generate_assets(&project)?;

    println!("Building project...\n");

    self.build(&project_path).with_context(|e| {
      format!("Failed to build project : {}", e)
    })?;

    println!("Executing tests...\n");

    let test_executable_path = format!("{}test/target/{}.test", project_path.display(), project.name());
    let test_arguments = Command::parse_arguments_from_cli(self, args);

    let mut child = ProcessCommand::new(&test_executable_path).args(test_arguments).spawn()?;
    child.wait()?;

    Ok(())
  }
}

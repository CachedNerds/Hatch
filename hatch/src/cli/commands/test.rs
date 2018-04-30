use std::process::Command as ProcessCommand;
use cli::commands::Command;
use hatch_error::{ HatchResult, ResultExt };
use clap::{ ArgMatches };
use generators::tup::Tup;
use generators::Generator;

pub struct Test;

impl<'test> Test {
  pub fn new() -> Test {
    Test
  }
}

impl<'command> Command<'command> for Test {
  fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
    let (project_path, project) = self.read_project_context(args)?;
    println!("Generating assets...\n");
    let generator: Box<Generator> = Box::new(Tup::new());
    self.generate_assets(generator, project_path.clone(), &project)?;
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

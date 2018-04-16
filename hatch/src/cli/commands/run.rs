use std::process::Command as ProcessCommand;
use cli::commands::Command;
use task;
use clap::{ ArgMatches };
use project::ProjectKind;
use hatch_error::Action;
use generators::tup::Tup;

pub struct Run;

impl<'run> Run {
  pub fn new() -> Run {
    Run
  }
}

impl<'command> Command<'command> for Run {
  fn execute(&self, args: &ArgMatches<'command>) -> Action {
    let project_path = self.project_path(args);
    let project = task::read_project(&project_path)?;
    match *project.kind() {
      ProjectKind::Binary => {
        println!("Generating assets...\n");
        let generator = Tup::boxed(&project);
        task::generate_assets(generator, &project)?;
        println!("Building project...\n");
        self.build(&project_path)?;
        println!("Executing...\n");
        let executable_path = format!("{}target/{}", project_path.display(), project.name());
        let run_arguments =  self.parse_arguments_from_cli(args);
        let mut child = ProcessCommand::new(&executable_path).args(run_arguments).spawn()?;
        child.wait()?;
        Ok(())
      }
      _ => Ok(())
    }
  }
}

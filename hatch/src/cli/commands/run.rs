use clap::ArgMatches;
use cli::commands::Command;
use failure::ResultExt;
use generators::tup::Tup;
use generators::Generator;
use hatch_error::Action;
use project::ProjectKind;
use std::process::Command as ProcessCommand;

pub struct Run;

impl<'run> Run {
    pub fn new() -> Run {
        Run
    }
}

impl<'command> Command<'command> for Run {
    fn execute(&self, generator: Box<Generator>, args: &ArgMatches<'command>) -> Action {
        let (project_path, project) = self.read_project_context(args)?;
        self.generate_assets(generator, project_path.clone(), &project)
            .with_context(|e| format!("asset generation failed : `{}`", e))?;

        match *project.kind() {
            ProjectKind::Binary => {
                println!("Generating assets...\n");
                let generator = Box::new(Tup {});
                self.generate_assets(generator, project_path.clone(), &project)?;
                println!("Building project...\n");
                self.build(&project_path)?;
                println!("Executing...\n");
                let executable_path =
                    format!("{}target/{}", project_path.display(), project.name());
                let run_arguments = self.parse_arguments_from_cli(args);
                let mut child = ProcessCommand::new(&executable_path)
                    .args(run_arguments)
                    .spawn()?;
                child.wait()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

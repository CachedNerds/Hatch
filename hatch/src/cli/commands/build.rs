use clap::ArgMatches;
use cli::commands::Command;
use generators::tup::Tup;
use generators::Generator;
use hatch_error::HatchResult;

pub struct Build;

impl<'build> Build {
    pub fn new() -> Build {
        Build
    }
}

impl<'command> Command<'command> for Build {
    fn execute(&self, generator: Box<Generator>, args: &ArgMatches<'command>) -> HatchResult<()> {
        let (project_path, project) = self.read_project_context(args)?;
        println!("Building project...\n");
        self.build(&project_path)?;
        println!("Generating assets...\n");
        // TODO: make a trait object so we can dynamic dispatch on a command line arg for generator
        let generator: Box<Generator> = Box::new(Tup {});
        self.generate_assets(generator, project_path, &project)?;
        Ok(())
    }
}

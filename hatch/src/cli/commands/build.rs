use clap::ArgMatches;
use cli::commands::Command;
use generators::tup::make_a_tup_in_a_box;
use hatch_error::HatchResult;

pub struct Build;

impl<'build> Build {
    pub fn new() -> Build {
        Build
    }
}

impl<'command> Command<'command> for Build {
    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
        let (project_path, project) = self.read_project_context(args)?;
        println!("Building project...\n");
        self.build(&project_path)?;
        println!("Generating assets...\n");
        // TODO: make a trait object so we can dynamic dispatch on a command line arg for generator
        let generator = make_a_tup_in_a_box();
        self.generate_assets(generator, project_path, &project)?;
        Ok(())
    }
}

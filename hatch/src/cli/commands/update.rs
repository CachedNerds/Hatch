use clap::ArgMatches;
use cli::commands::Command;
use generators::tup::Tup;
use generators::Generator;
use hatch_error::HatchResult;

pub struct Update;

impl<'update> Update {
    pub fn new() -> Update {
        Update
    }
}

impl<'command> Command<'command> for Update {
    fn execute(&self, generator: Box<Generator>, args: &ArgMatches<'command>) -> HatchResult<()> {
        let (path, project) = self.read_project_context(args)?;
        self.generate_assets(generator, path.clone(), &project)?;
        Ok(())
    }
}

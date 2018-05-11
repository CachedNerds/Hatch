use clap::ArgMatches;
use cli::commands::Command;
use generators::tup::make_a_tup_in_a_box;
use hatch_error::HatchResult;

pub struct Update;

impl<'update> Update {
    pub fn new() -> Update {
        Update
    }
}

impl<'command> Command<'command> for Update {
    fn execute(&self, args: &ArgMatches<'command>) -> HatchResult<()> {
        let (path, project) = self.read_project_context(args)?;
        let generator = make_a_tup_in_a_box();
        self.generate_assets(generator, path.clone(), &project)?;
        Ok(())
    }
}
